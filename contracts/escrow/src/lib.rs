#![no_std]
mod events;
mod types;
pub mod oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/signaturit_oracle.wasm"
    );
    pub type OracleClient<'a> = Client<'a>;
}

pub mod notes_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/notes_nft.wasm"
    );
    pub type NotesNFTClient<'a> = Client<'a>;
}

use events::EscrowEvent;
use oracle_traits::OracleConsumer;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, panic_with_error, token, vec, Address, Env, IntoVal, String, Symbol,
};
use storage::Storage;
use types::{
    DataKey, EscrowError, EscrowProposal, NullableString, ProposalStatus, SignatureStatus,
    SignatureTxEscrow,
};

fn check_initialization(env: &Env) {
    if !DataKey::AssetAddress.has(env) || !DataKey::OracleAddress.has(env) {
        panic_with_error!(env, EscrowError::NotInit);
    }
}

fn get_oracle(env: &Env) -> Address {
    DataKey::OracleAddress.get(env).unwrap()
}

fn get_asset(env: &Env) -> Address {
    DataKey::AssetAddress.get(env).unwrap()
}

fn get_nft(env: &Env) -> Address {
    DataKey::NFTNotesAddress.get(env).unwrap()
}

fn transfer_funds(env: &Env, from: &Address, to: &Address, amount: &i128) {
    let asset_address = get_asset(env);
    let client = token::Client::new(env, &asset_address);
    client.transfer(from, to, amount);
}

fn get_proposal(env: &Env, escrow_id: String) -> EscrowProposal {
    if !DataKey::Proposal(escrow_id.clone()).has(&env) {
        panic_with_error!(&env, EscrowError::ProposalNotFound);
    }

    DataKey::Proposal(escrow_id).get(env).unwrap()
}

fn get_signature_tx_escrow(env: &Env, signaturit_id: String) -> SignatureTxEscrow {
    if !DataKey::SignatureProcess(signaturit_id.clone()).has(&env) {
        panic_with_error!(&env, EscrowError::SignatureProcessNotFound);
    }

    DataKey::SignatureProcess(signaturit_id).get(&env).unwrap()
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn get_oracle(env: Env) -> Address {
        check_initialization(&env);
        get_oracle(&env)
    }

    pub fn get_asset(env: Env) -> Address {
        check_initialization(&env);
        get_asset(&env)
    }

    pub fn get_nft_notes(env: Env) -> Address {
        check_initialization(&env);
        get_nft(&env)
    }

    pub fn get_proposal(env: Env, escrow_id: String) -> EscrowProposal {
        get_proposal(&env, escrow_id)
    }

    pub fn get_signature_tx_escrow(env: Env, signaturit_id: String) -> SignatureTxEscrow {
        get_signature_tx_escrow(&env, signaturit_id)
    }

    pub fn initialize(
        env: Env,
        asset_address: Address,
        oracle_address: Address,
        nft_notes_address: Address,
    ) {
        if DataKey::AssetAddress.has(&env)
            && DataKey::OracleAddress.has(&env)
            && DataKey::NFTNotesAddress.has(&env)
        {
            panic_with_error!(env, EscrowError::AlreadyInitialized);
        }

        DataKey::AssetAddress.set(&env, &asset_address);
        DataKey::OracleAddress.set(&env, &oracle_address);
        DataKey::NFTNotesAddress.set(&env, &nft_notes_address);

        // Emit the Initialized event
        EscrowEvent::Initialized(asset_address, oracle_address, nft_notes_address).publish(&env);
    }

    /**
     * Register a new proposal to be picked,
     */
    pub fn add_proposal(
        env: Env,
        stocken_proposal_id: String,
        proposer_address: Address,
        min_funds: i128,
    ) {
        check_initialization(&env);

        if DataKey::Proposal(stocken_proposal_id.clone()).has(&env) {
            panic_with_error!(&env, EscrowError::AlreadyProposed);
        }

        let propose = EscrowProposal {
            escrow_id: stocken_proposal_id.clone(),
            owner: proposer_address.clone(),
            status: ProposalStatus::Actived,
            min_funds,
            signature_tx_linked: NullableString::None,
        };

        // Save the proposal
        DataKey::Proposal(stocken_proposal_id).set(&env, &propose);

        // Emit the NewProposal event
        EscrowEvent::NewProposal(propose.escrow_id, propose.owner).publish(&env);
    }

    pub fn register_escrow(
        env: Env,
        proposal_id: String,
        signaturit_id: String,
        sender_id: Address,
        funds: i128,
    ) {
        check_initialization(&env);

        let mut propose: EscrowProposal = get_proposal(&env, proposal_id.clone());

        if DataKey::SignatureProcess(signaturit_id.clone()).has(&env) {
            panic_with_error!(&env, EscrowError::SignatureProcessExist);
        }

        if propose.status != ProposalStatus::Actived {
            panic_with_error!(&env, EscrowError::PickedOrCanceled);
        }

        if funds < propose.min_funds {
            panic_with_error!(&env, EscrowError::NoEnoughtFunds);
        }

        transfer_funds(&env, &sender_id, &env.current_contract_address(), &funds);

        // Get the oracle client
        let oracle_client = oracle::OracleClient::new(&env, &get_oracle(&env));

        // Grant auth for calling the function
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: get_oracle(&env),
                    fn_name: Symbol::new(&env, "register_new_signature_process"),
                    args: (env.current_contract_address(), signaturit_id.clone()).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Get the oracle id for this process signature
        let oracle_id = oracle_client
            .register_new_signature_process(&env.current_contract_address(), &signaturit_id);

        let tx_register = SignatureTxEscrow {
            id: signaturit_id.clone(),
            propose_id: propose.escrow_id.clone(),
            oracle_id,
            buyer: sender_id,
            receiver: propose.owner.clone(),
            funds,
            status: SignatureStatus::Progress,
            nft_proof_id: None,
        };

        // This way, the propose can be picked just once per time
        propose.status = ProposalStatus::Picked;
        propose.signature_tx_linked = NullableString::Some(signaturit_id.clone());
        DataKey::Proposal(proposal_id).set(&env, &propose);
        DataKey::SignatureProcess(signaturit_id).set(&env, &tx_register);

        // Emit the RegisterEscrow event
        EscrowEvent::RegisterEscrow(
            tx_register.id,
            tx_register.propose_id,
            tx_register.oracle_id,
            tx_register.buyer,
            tx_register.funds,
        )
        .publish(&env);
    }
}

#[contractimpl]
impl OracleConsumer for EscrowContract {
    fn completed_signature(env: Env, signaturit_id: String, document_hash: String) {
        check_initialization(&env);
        get_oracle(&env).require_auth();

        let mut signature_process = get_signature_tx_escrow(&env, signaturit_id.clone());

        let mut propose = get_proposal(&env, signature_process.clone().propose_id);

        // Release the funds to the owner of the propose
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: get_asset(&env),
                    fn_name: Symbol::new(&env, "transfer"),
                    args: (
                        env.current_contract_address(),
                        signature_process.receiver.clone(),
                        signature_process.funds.clone(),
                    )
                        .into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);
        transfer_funds(
            &env,
            &env.current_contract_address(), // from
            &signature_process.receiver,     // to
            &signature_process.funds,        // amount
        );

        // Get the NFT client
        let nft_client = notes_nft::NotesNFTClient::new(&env, &get_nft(&env));

        // Grant auth for calling the function
        // Mint to NFT for the "buyer" address since is the address that gave the funds
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: get_nft(&env),
                    fn_name: Symbol::new(&env, "mint"),
                    args: (signature_process.buyer.clone(), document_hash.clone()).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // NFT ID minted
        let token_id_minted: u32 = nft_client.mint(&signature_process.buyer, &document_hash);

        signature_process.nft_proof_id = Some(token_id_minted);
        signature_process.status = SignatureStatus::Completed;
        DataKey::SignatureProcess(signaturit_id.clone()).set(&env, &signature_process);

        propose.status = ProposalStatus::Completed;
        DataKey::Proposal(propose.escrow_id.clone()).set(&env, &propose);

        // Emit the SignedCompleted event
        EscrowEvent::SignedCompleted(
            signature_process.id,
            signature_process.propose_id,
            signature_process.buyer,
            signature_process.receiver,
            signature_process.funds,
            token_id_minted,
        )
        .publish(&env);
    }

    fn failed_signature(env: Env, signaturit_id: String) {
        check_initialization(&env);
        get_oracle(&env).require_auth();

        let mut signature_process = get_signature_tx_escrow(&env, signaturit_id.clone());

        let mut propose = get_proposal(&env, signature_process.clone().propose_id);

        // Return the funds to the address that picked the propose
        transfer_funds(
            &env,
            &env.current_contract_address(), // from
            &signature_process.buyer,        // to
            &signature_process.funds,        // amount
        );

        signature_process.status = SignatureStatus::Canceled;
        DataKey::SignatureProcess(signature_process.id.clone()).set(&env, &signature_process);

        propose.status = ProposalStatus::Actived;
        propose.signature_tx_linked = NullableString::None;
        DataKey::Proposal(propose.escrow_id.clone()).set(&env, &propose);

        // Emit the SignedFailed event
        EscrowEvent::SignedFailed(
            signature_process.id,
            signature_process.propose_id,
            signature_process.buyer,
        )
        .publish(&env);
    }
}

mod test;
