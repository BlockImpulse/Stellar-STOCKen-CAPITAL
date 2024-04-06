#![no_std]
mod storage;
mod types;

use storage::Storage;
use types::{DataKey, EscrowProposal, ProposalStatus};

use soroban_sdk::{contract, contractimpl, log, BytesN, Env, Symbol};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /**
     * Register a new proposal to be picked,
     */
    pub fn add_proposal(env: Env, id: BytesN<32>) {
        let propose_0 = DataKey::Proposal(id.clone()).has(&env);
        log!(&env, "is_proposed_0:", propose_0);

        let proposal: EscrowProposal = EscrowProposal {
            id: id.clone(),
            status: ProposalStatus::Actived,
        };

        DataKey::Proposal(id.clone()).set(&env, &proposal);

        let propose_1 = DataKey::Proposal(id.clone()).has(&env);
        log!(&env, "is_proposed_1:", propose_1);
    }

    /**
     * This will pick an ID and offer XML
     */
    pub fn pick_proposal(env: Env, signaturit_id: BytesN<32>) {
        //
    }
}

mod test;
