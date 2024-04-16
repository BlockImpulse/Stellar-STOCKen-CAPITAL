#![no_std]
mod events;
mod storage;
mod types;

use events::{INIT_TOPIC, PROPOSAL_TOPIC, REGISTER_TOPIC, SUCCESS_SIGN_TOPIC};
use storage::Storage;
use types::{
    DataKey, EscrowError, EscrowProposal, ProposalStatus, SignatureStatus, SignatureTxEscrow,
};

use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, token, Address, Env, String,
};

fn check_initialization(env: &Env) {
    if !DataKey::AssetAddress.has(env) || !DataKey::OracleAddress.has(env) {
        panic_with_error!(env, EscrowError::NotInit);
    }
}

fn only_oracle(env: &Env, caller_address: Address) {
    let oracle_address: Address = DataKey::OracleAddress.get(env).unwrap();

    if !caller_address.eq(&oracle_address) {
        panic_with_error!(env, EscrowError::OnlyOracle);
    }
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
        DataKey::AssetAddress.set(&env, &asset_address);
        DataKey::OracleAddress.set(&env, &oracle_address);

        env.events()
            .publish((INIT_TOPIC,), (asset_address, oracle_address));
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

        // Require auth of the sender to lock the funds
        // Move the funds to here
        // Call this at end?? before emit the event
        sender_id.require_auth();
        transfer_funds(&env, &sender_id, &env.current_contract_address(), &funds);

        // TODO: Call the Oracle and get the oracle id to identify the tx escrow
        // It will fail if the signaturit id is already picked
        // let oracle_id = oracle.register_new_sign(signaturit_id);
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
        DataKey::Proposal(proposal_id).set(&env, &propose);
        DataKey::SignatureProcess(signaturit_id).set(&env, &tx_register);
    }

    pub fn success_signature(env: Env, caller_address: Address, signaturit_id: String) {
        // TODO: Only the Oracle can call this function
        // Idk the correc to do this here since we have require_auth()
        // When the register is made, the contract should grant auth to the Oracle address
        caller_address.require_auth();

        // OracleADdress.require_auth();

        // Only oracle can call
        only_oracle(&env, caller_address);

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
        env.events().publish(
            (SUCCESS_SIGN_TOPIC, symbol_short!("Completed")),
            (signaturit_id, propose.escrow_id),
        );
    }
}

mod test;
