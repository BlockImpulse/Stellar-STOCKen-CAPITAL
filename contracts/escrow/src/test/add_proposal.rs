#![cfg(test)]

use crate::{
    events::{ADDED_TOPIC, PROPOSAL_TOPIC},
    test::{escrow::EscrowError, EscrowTest, STOCKEN_ID_1, STOCKEN_ID_2},
    types::NullableString,
    EscrowProposal, ProposalStatus,
};
use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, IntoVal, String,
};

#[test]
fn add_proposal_not_initialized() {
    let test = EscrowTest::setup_non_init();

    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens

    let res = test
        .escrow
        .try_add_proposal(&stocken_id, &test.alice, &amount_asked);

    assert_eq!(res, Err(Ok(EscrowError::NotInit.into())));
}

#[test]
fn add_proposal() {
    let test = EscrowTest::setup();

    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens

    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    let expected_proposal = EscrowProposal {
        escrow_id: stocken_id,
        owner: test.alice,
        status: ProposalStatus::Actived,
        min_funds: amount_asked,
        signature_tx_linked: NullableString::None,
    };

    let event_expected = (
        test.escrow.address.clone(),
        (PROPOSAL_TOPIC, ADDED_TOPIC).into_val(&test.env),
        expected_proposal.into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "initialized event not present"
    );
}

#[test]
fn test_add_multiple_proposal() {
    let test = EscrowTest::setup();

    // Data for proposals
    let stocken_id_1 = String::from_str(&test.env, STOCKEN_ID_1);
    let proposer_address_1 = Address::generate(&test.env);
    let amount_asked_1: i128 = 9_000_000_000_000_000_000;

    let stocken_id_2 = String::from_str(&test.env, STOCKEN_ID_2);
    let proposer_address_2 = Address::generate(&test.env);
    let amount_asked_2: i128 = 18_000_000_000_000_000_000;

    test.escrow
        .add_proposal(&stocken_id_1, &proposer_address_1, &amount_asked_1);
    test.escrow
        .add_proposal(&stocken_id_2, &proposer_address_2, &amount_asked_2);

    let expected_proposal_1 = EscrowProposal {
        escrow_id: stocken_id_1,
        owner: proposer_address_1,
        status: ProposalStatus::Actived,
        min_funds: amount_asked_1,
        signature_tx_linked: NullableString::None,
    };

    let expected_proposal_2 = EscrowProposal {
        escrow_id: stocken_id_2,
        owner: proposer_address_2,
        status: ProposalStatus::Actived,
        min_funds: amount_asked_2,
        signature_tx_linked: NullableString::None,
    };

    let event_expected_1 = (
        test.escrow.address.clone(),
        (PROPOSAL_TOPIC, ADDED_TOPIC).into_val(&test.env),
        expected_proposal_1.into_val(&test.env),
    );

    let event_expected_2 = (
        test.escrow.address,
        (PROPOSAL_TOPIC, ADDED_TOPIC).into_val(&test.env),
        expected_proposal_2.into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected_1),
        "Wrong event data emitted"
    );

    assert!(
        test.env.events().all().contains(event_expected_2),
        "Wrong event data emitted"
    );
}
