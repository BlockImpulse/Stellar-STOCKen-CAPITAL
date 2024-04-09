#![cfg(test)]

extern crate std;

use crate::{EscrowContract, EscrowContractClient, EscrowProposal, ProposalStatus};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    Address, Env, IntoVal, String, U256,
};

// keccak256(STOCKEN_ID_1)
const STOCKEN_ID_1: &str = "6ef7e237bbddb133bb3504cad9e2ec7ff90c0c9b63567a632dbad8bb2b923728";
// keccak256(STOCKEN_ID_2)
const STOCKEN_ID_2: &str = "af8f0b8ba4749d7a83edcd03a18e3ee3807fca630f8a18e8e59be53ea15c9e95";

#[test]
fn test_add_proposal() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &contract_id);

    let stocken_id = String::from_str(&env, STOCKEN_ID_1);
    let proposer_address = Address::generate(&env);
    let amount_asked = U256::from_u32(&env, 900000u32);

    escrow_client.add_proposal(&stocken_id, &proposer_address, &amount_asked);

    let expected_proposal = EscrowProposal {
        escrow_id: stocken_id,
        owner: proposer_address,
        status: ProposalStatus::Actived,
        min_funds: amount_asked,
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

#[test]
fn test_add_multiple_proposal() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &contract_id);

    // Data for proposals
    let stocken_id_1 = String::from_str(&env, STOCKEN_ID_1);
    let proposer_address_1 = Address::generate(&env);
    let amount_asked_1 = U256::from_u32(&env, 900000u32);

    let stocken_id_2 = String::from_str(&env, STOCKEN_ID_2);
    let proposer_address_2 = Address::generate(&env);
    let amount_asked_2 = U256::from_u32(&env, 1800000u32);

    escrow_client.add_proposal(&stocken_id_1, &proposer_address_1, &amount_asked_1);
    escrow_client.add_proposal(&stocken_id_2, &proposer_address_2, &amount_asked_2);

    let expected_proposal_1 = EscrowProposal {
        escrow_id: stocken_id_1,
        owner: proposer_address_1,
        status: ProposalStatus::Actived,
        min_funds: amount_asked_1,
    };

    let expected_proposal_2 = EscrowProposal {
        escrow_id: stocken_id_2,
        owner: proposer_address_2,
        status: ProposalStatus::Actived,
        min_funds: amount_asked_2,
    };

    let event_expected_1 = (
        contract_id.clone(),
        (symbol_short!("Proposal"), symbol_short!("Added")).into_val(&env),
        expected_proposal_1.into_val(&env),
    );

    let event_expected_2 = (
        contract_id.clone(),
        (symbol_short!("Proposal"), symbol_short!("Added")).into_val(&env),
        expected_proposal_2.into_val(&env),
    );

    assert!(
        env.events().all().contains(event_expected_1),
        "Wrong event data emitted"
    );

    assert!(
        env.events().all().contains(event_expected_2),
        "Wrong event data emitted"
    );
}
