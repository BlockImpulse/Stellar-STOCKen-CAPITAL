#![no_std]
mod events;
mod storage;
mod types;

use events::PROPOSAL_TOPIC;
use storage::Storage;
use types::{DataKey, EscrowError, EscrowProposal, ProposalStatus};

use soroban_sdk::{contract, contractimpl, panic_with_error, symbol_short, Address, Env, String};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    // fn initialize() {}

    /**
     * Register a new proposal to be picked,
     */
    pub fn add_proposal(env: Env, stocken_proposal_id: String, proposer_address: Address) {
        let escrow_id: u32 = DataKey::ProposalCounter.get(&env).unwrap_or(0);

        if DataKey::Proposal(escrow_id.clone()).has(&env) {
            panic_with_error!(&env, EscrowError::AlreadyProposed);
        }

        let propose = EscrowProposal {
            escrow_id: escrow_id.clone(),
            stocken_proposal_id: stocken_proposal_id.clone(),
            owner: proposer_address,
            status: ProposalStatus::Actived,
        };

        env.events()
            .publish((PROPOSAL_TOPIC, symbol_short!("Added")), propose.clone());

        // Save the proposal
        DataKey::Proposal(escrow_id).set(&env, &propose);
    }

    pub fn get_proposal(env: &Env, escrow_id: u32) -> EscrowProposal {
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
    pub fn register_escrow(env: Env, signaturit_id: String) {}
}

mod test;
