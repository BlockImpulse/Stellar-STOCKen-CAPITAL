#![no_std]
mod events;
mod storage;
mod types;

use events::{INIT_TOPIC, PROPOSAL_TOPIC, REGISTER_TOPIC};
use storage::Storage;
use types::{DataKey, EscrowError, EscrowProposal, ProposalStatus, SignatureTxEscrow};

use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, token, Address, Env, String,
};

fn check_initialization(env: &Env) {
    if !DataKey::AssetAddress.has(env) || !DataKey::OracleAddress.has(env) {
        panic_with_error!(env, EscrowError::NotInit);
    }
}

fn transfer_funds(env: &Env, from: &Address, to: &Address, amount: &i128) {
    let asset_address: &Address = &DataKey::AssetAddress.get(env).unwrap();
    let client = token::Client::new(env, asset_address);
    client.transfer(from, to, amount);
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

    pub fn get_proposal(env: &Env, escrow_id: String) -> EscrowProposal {
        if !DataKey::Proposal(escrow_id.clone()).has(&env) {
            panic_with_error!(&env, EscrowError::ProposalNotFound);
        }

        DataKey::Proposal(escrow_id).get(env).unwrap()
    }

    /*
    TODO: The `signaturit_id` is expected to be an UUID String. This need more
    testing and work since String is case sensitive. Two string that represent
    the same UUID but with some hex values with a uppercase or lowercase, it
    will lead to the contract to identify it as two diff Ids.
    */
    pub fn register_escrow(
        env: Env,
        proposal_id: String,
        signaturit_id: String,
        sender_id: Address,
        funds: i128,
    ) {
        check_initialization(&env);

        let mut propose: EscrowProposal = Self::get_proposal(&env, proposal_id.clone());

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
        // let oracle_id = oracle.register_new_sign(signaturit_id);
        let oracle_id = 0u32;

        let tx_register = SignatureTxEscrow {
            id: signaturit_id.clone(),
            propose_id: propose.escrow_id.clone(),
            oracle_id,
            buyer: sender_id,
            receiver: propose.owner.clone(),
            funds,
        };

        env.events()
            .publish((REGISTER_TOPIC, symbol_short!("New")), tx_register.clone());

        // TODO: Change the status to picked to avoid multiple picks to single propose.
        // Maybe we can do some kind of record for each time of a `proposal_id` is picked
        // Then the oracle callback will cancel all the signatures process.
        // But this means that the event reader have to make the API call to signaturit
        // and I don't know if it's ok.
        //
        // This way, the propose can be picked just once per time
        propose.status = ProposalStatus::Picked;
        DataKey::Proposal(proposal_id).set(&env, &propose);
        DataKey::SignatureProcess(signaturit_id).set(&env, &tx_register);
    }
}

mod test;
