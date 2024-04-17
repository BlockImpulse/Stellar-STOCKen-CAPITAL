#![no_std]
mod events;
mod storage;
mod types;

use events::{
    INITIALIZED_TOPIC, PROPOSAL_TOPIC, REGISTER_TOPIC, SIGNED_COMPLETED_TOPIC, SIGNED_FAILED_TOPIC,
};
use storage::Storage;
use types::{
    DataKey, EscrowError, EscrowProposal, NullableString, ProposalStatus, SignatureStatus,
    SignatureTxEscrow,
};

use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, panic_with_error, symbol_short, token, vec, Address, Env, IntoVal,
    String, Symbol,
};

fn check_initialization(env: &Env) {
    if !DataKey::AssetAddress.has(env) || !DataKey::OracleAddress.has(env) {
        panic_with_error!(env, EscrowError::NotInit);
    }
}

fn get_oracle(env: &Env) -> Address {
    DataKey::OracleAddress.get(env).unwrap()
}

fn transfer_funds(env: &Env, from: &Address, to: &Address, amount: &i128) {
    let asset_address: &Address = &DataKey::AssetAddress.get(env).unwrap();
    let client = token::Client::new(env, asset_address);
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
    pub fn initialize(env: Env, asset_address: Address, oracle_address: Address) {
        // TODO: Check initialization of. Cannot reinitilize
        DataKey::AssetAddress.set(&env, &asset_address);
        DataKey::OracleAddress.set(&env, &oracle_address);

        env.events()
            .publish((INITIALIZED_TOPIC,), (asset_address, oracle_address));
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
            owner: proposer_address,
            status: ProposalStatus::Actived,
            min_funds,
            signature_tx_linked: NullableString::None,
        };

        env.events()
            .publish((PROPOSAL_TOPIC, symbol_short!("Added")), propose.clone());

        // Save the proposal
        DataKey::Proposal(stocken_proposal_id).set(&env, &propose);
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

        if propose.status != ProposalStatus::Actived {
            panic_with_error!(&env, EscrowError::PickedOrCanceled);
        }

        if funds < propose.min_funds {
            panic_with_error!(&env, EscrowError::NoEnoughtFunds);
        }

        sender_id.require_auth();
        transfer_funds(&env, &sender_id, &env.current_contract_address(), &funds);

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

        // TODO: Call the Oracle and get the oracle id to identify the tx escrow
        // It will fail if the signaturit id is already picked
        // let oracle_id = oracle.register_new_signature_process(env.current_contract_address(), signaturit_id.clone());
        let oracle_id = 0u32;

        let tx_register = SignatureTxEscrow {
            id: signaturit_id.clone(),
            propose_id: propose.escrow_id.clone(),
            oracle_id,
            buyer: sender_id,
            receiver: propose.owner.clone(),
            funds,
            status: SignatureStatus::Progress,
        };

        env.events()
            .publish((REGISTER_TOPIC, symbol_short!("New")), tx_register.clone());

        // This way, the propose can be picked just once per time
        propose.status = ProposalStatus::Picked;
        propose.signature_tx_linked = NullableString::Some(signaturit_id.clone());
        DataKey::Proposal(proposal_id).set(&env, &propose);
        DataKey::SignatureProcess(signaturit_id).set(&env, &tx_register);
    }

    pub fn completed_signature(env: Env, signaturit_id: String) {
        get_oracle(&env).require_auth();

        let mut signature_process = get_signature_tx_escrow(&env, signaturit_id.clone());

        let mut propose = get_proposal(&env, signature_process.propose_id);

        // Release the funds to the owner of the propose
        transfer_funds(
            &env,
            &env.current_contract_address(), // from
            &signature_process.receiver,     // to
            &signature_process.funds,        // amount
        );

        signature_process.status = SignatureStatus::Completed;
        propose.status = ProposalStatus::Completed;

        // TODO: Mint the NFT with the signature ID and the propose ID
        env.events()
            .publish(SIGNED_COMPLETED_TOPIC, (signaturit_id, propose.escrow_id));
    }

    pub fn failed_signature(env: Env, signaturit_id: String) {
        get_oracle(&env).require_auth();

        // Only oracle can call
        // only_oracle(&env, caller_address);

        let mut signature_process = get_signature_tx_escrow(&env, signaturit_id.clone());

        let mut propose = get_proposal(&env, signature_process.propose_id);

        // Return the funds to the address that picked the propose
        transfer_funds(
            &env,
            &env.current_contract_address(), // from
            &signature_process.buyer,        // to
            &signature_process.funds,        // amount
        );

        signature_process.status = SignatureStatus::Canceled;
        propose.status = ProposalStatus::Actived;
        propose.signature_tx_linked = NullableString::None;

        env.events().publish(
            SIGNED_FAILED_TOPIC,
            (signature_process.id, propose.escrow_id),
        );
    }
}

mod test;
