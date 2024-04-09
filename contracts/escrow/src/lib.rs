#![no_std]
mod events;
mod storage;
mod types;

use events::{PROPOSAL_TOPIC, REGISTER_TOPIC};
use storage::Storage;
use types::{DataKey, EscrowError, EscrowProposal, ProposalStatus, SignatureTxEscrow};

use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, Address, Env, String, U256,
};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    // fn initialize() {}

    /**
     * Register a new proposal to be picked,
     */
    pub fn add_proposal(
        env: Env,
        stocken_proposal_id: String,
        proposer_address: Address,
        min_funds: U256,
    ) {
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
    pub fn register_escrow(env: Env, signaturit_id: String, sender_id: Address, funds: U256) {
        let propose: EscrowProposal = Self::get_proposal(&env, signaturit_id.clone());

        if propose.status != ProposalStatus::Actived {
            panic_with_error!(&env, EscrowError::PickedOrCanceled);
        }

        if propose.min_funds < funds {
            panic_with_error!(&env, EscrowError::NoEnoughtFunds);
        }

        // Require auth of the sender to lock the funds
        sender_id.require_auth();
        // TODO: Move the funds to the contract

        // Move the funds to here

        // TODO: Call the Oracle and get the oracle id to identify the tx escrow
        // let oracle_id = oracle.register_new_sign(signaturit_id);
        let oracle_id = 0u32;

        let tx_register = SignatureTxEscrow {
            id: signaturit_id.clone(),
            oracle_id,
            buyer: sender_id,
            receiver: propose.owner,
            funds,
        };

        env.events()
            .publish((REGISTER_TOPIC, symbol_short!("New")), tx_register.clone());

        DataKey::SignatureProcess(signaturit_id).set(&env, &tx_register);
    }
}

mod test;
