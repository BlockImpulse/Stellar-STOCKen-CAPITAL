#![cfg(test)]

extern crate std;

use super::{EscrowContract, EscrowContractClient, EscrowProposal, ProposalStatus};
use soroban_sdk::IntoVal;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    Address, Env, String,
};

// keccak256(STOCKEN_ID_1)
const STOCKEN_ID_1: &str = "6ef7e237bbddb133bb3504cad9e2ec7ff90c0c9b63567a632dbad8bb2b923728";
// keccak256(STOCKEN_ID_2)
const _STOCKEN_ID_2: &str = "af8f0b8ba4749d7a83edcd03a18e3ee3807fca630f8a18e8e59be53ea15c9e95";
// keccak256(STOCKEN_ID_3)
const _STOCKEN_ID_3: &str = "b04803e756cd42e63238ff15658eac8b869c8318991a9c686005e2f5ebd28bc7";

#[test]
fn test_add_proposal() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &contract_id);

    let stocken_id = String::from_str(&env, STOCKEN_ID_1);
    let proposer_address = Address::generate(&env);

    escrow_client.add_proposal(&stocken_id, &proposer_address);

    let expected_proposal = EscrowProposal {
        escrow_id: 0u32,
        stocken_proposal_id: stocken_id,
        owner: proposer_address,
        status: ProposalStatus::Actived,
    };

    let event_expected = (
        contract_id.clone(),
        (symbol_short!("Proposal"), symbol_short!("Added")).into_val(&env),
        expected_proposal.into_val(&env),
    );

    assert!(
        env.events().all().contains(event_expected),
        "Wrong event data emitted"
    );
}
