#![cfg(test)]

extern crate std;
use core::ops::{Div, Mul};
use std::string::ToString;

use crate::{
    events::EscrowEvent2,
    test::{escrow::EscrowError, EscrowTest, STOCKEN_ID_1, STOCKEN_ID_2},
};
use soroban_sdk::{
    testutils::{Events, MockAuth, MockAuthInvoke},
    IntoVal, String,
};
use uuid::Uuid;

#[test]
fn new_register_not_initialized() {
    let test = EscrowTest::setup_non_init();

    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = 20_000_000_000_000_000_000; // 20 tokens

    let balance_before_bob = test.token.balance(&test.bob);

    let res = test.escrow.mock_all_auths().try_register_escrow(
        &stocken_id,
        &signaturit_id,
        &test.bob,
        &amount_to_give,
    );

    assert_eq!(res, Err(Ok(EscrowError::NotInit.into())));

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        0,
        "escrow contract receive funds incorrectly"
    );
}

#[test]
fn new_register_proposal_not_found() {
    let test = EscrowTest::setup();

    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = 20_000_000_000_000_000_000; // 20 tokens

    let balance_before_bob = test.token.balance(&test.bob);

    let res = test.escrow.mock_all_auths().try_register_escrow(
        &stocken_id,
        &signaturit_id,
        &test.bob,
        &amount_to_give,
    );

    assert_eq!(res, Err(Ok(EscrowError::ProposalNotFound.into())));

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        0,
        "escrow contract receive funds incorrectly"
    );
}

#[test]
fn new_register_not_enough_funds() {
    let test = EscrowTest::setup();

    // Add a proposal
    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens
    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    // Pick a escrow
    // The sginaturit ID is an UUID
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = amount_asked.div(2);

    let balance_before_alice = test.token.balance(&test.alice);
    let balance_before_bob = test.token.balance(&test.bob);

    let res = test.escrow.mock_all_auths().try_register_escrow(
        &stocken_id,
        &signaturit_id,
        &test.bob,
        &amount_to_give,
    );

    assert_eq!(res, Err(Ok(EscrowError::NoEnoughtFunds.into())));

    assert_eq!(
        test.token.balance(&test.alice),
        balance_before_alice,
        "proposal owner balance is not correct"
    );
    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        0,
        "escrow contract receive funds incorrectly"
    );
}

#[test]
fn new_register_proposal_already_picked() {
    let test = EscrowTest::setup();

    // Add a proposal
    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens
    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    // Pick a escrow
    // The sginaturit ID is an UUID
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = amount_asked.mul(2);

    let balance_before_alice = test.token.balance(&test.alice);
    let balance_before_bob = test.token.balance(&test.bob);

    test.escrow
        .mock_auths(&[MockAuth {
            address: &test.bob,
            invoke: &MockAuthInvoke {
                contract: &test.token.address,
                fn_name: "transfer",
                args: (
                    test.bob.clone(),
                    test.escrow.address.clone(),
                    amount_to_give.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .register_escrow(&stocken_id, &signaturit_id, &test.bob, &amount_to_give);

    // The proposal owner should not receive the funds until the signature
    // process is completed
    assert_eq!(
        test.token.balance(&test.alice),
        balance_before_alice,
        "proposal owner balance has changed"
    );

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob - amount_to_give,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        amount_to_give,
        "escrow contract does not have the correct balance"
    );

    let event_expected = (
        test.escrow.address.clone(),
        (EscrowEvent2::RegisterEscrow(
            signaturit_id.clone(),
            stocken_id.clone(),
            0u32,
            test.bob.clone(),
            amount_to_give,
        )
        .name(),)
            .into_val(&test.env),
        (
            signaturit_id,
            stocken_id.clone(),
            0u32,
            &test.bob,
            amount_to_give,
        )
            .into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "register escrow event not present"
    );

    // The proposal was picked correctly.

    // Made a new register with same proposal
    let signaturit_id_2 = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give_2: i128 = amount_asked.mul(4); // Does not matter if it's higher

    let res = test.escrow.mock_all_auths().try_register_escrow(
        &stocken_id,
        &signaturit_id_2,
        &test.bob,
        &amount_to_give_2,
    );

    assert_eq!(res, Err(Ok(EscrowError::PickedOrCanceled.into())));
}

#[test]
fn new_register_signature_process_exist() {
    let test = EscrowTest::setup();

    // Add a proposal
    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens
    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    // Pick a escrow
    // The sginaturit ID is an UUID
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = amount_asked.mul(2);

    let balance_before_alice = test.token.balance(&test.alice);
    let balance_before_bob = test.token.balance(&test.bob);

    test.escrow
        .mock_auths(&[MockAuth {
            address: &test.bob,
            invoke: &MockAuthInvoke {
                contract: &test.token.address,
                fn_name: "transfer",
                args: (
                    test.bob.clone(),
                    test.escrow.address.clone(),
                    amount_to_give.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .register_escrow(&stocken_id, &signaturit_id, &test.bob, &amount_to_give);

    // The proposal owner should not receive the funds until the signature
    // process is completed
    assert_eq!(
        test.token.balance(&test.alice),
        balance_before_alice,
        "proposal owner balance has changed"
    );

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob - amount_to_give,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        amount_to_give,
        "escrow contract does not have the correct balance"
    );

    let event_expected = (
        test.escrow.address.clone(),
        (EscrowEvent2::RegisterEscrow(
            signaturit_id.clone(),
            stocken_id.clone(),
            0u32,
            test.bob.clone(),
            amount_to_give,
        )
        .name(),)
            .into_val(&test.env),
        (
            signaturit_id.clone(),
            stocken_id.clone(),
            0u32,
            &test.bob,
            amount_to_give,
        )
            .into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "register escrow event not present"
    );

    // Make a new proposal but when register the escrow, use the same previous
    // signaturit id
    // Add a proposal
    let stocken_id = String::from_str(&test.env, STOCKEN_ID_2);
    let amount_asked: i128 = 30_000_000_000_000_000_000; // 30 tokens

    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    //

    // Pick a escrow
    // The sginaturit ID is an UUID
    let same_signaturit_id = signaturit_id;
    let amount_to_give_2: i128 = amount_asked;

    let balance_before_alice = test.token.balance(&test.alice);
    let balance_before_bob = test.token.balance(&test.bob);
    let balance_before_escrow = test.token.balance(&test.escrow.address);

    let res = test
        .escrow
        .mock_auths(&[MockAuth {
            address: &test.bob,
            invoke: &MockAuthInvoke {
                contract: &test.token.address,
                fn_name: "transfer",
                args: (
                    test.bob.clone(),
                    test.escrow.address.clone(),
                    amount_to_give_2.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_register_escrow(
            &stocken_id,
            &same_signaturit_id,
            &test.bob,
            &amount_to_give_2,
        );

    // The contract call will fail since the signaturit id already exist
    assert_eq!(res, Err(Ok(EscrowError::SignatureProcessExist.into())));

    assert_eq!(
        test.token.balance(&test.alice),
        balance_before_alice,
        "proposal owner balance has changed"
    );

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        balance_before_escrow,
        "escrow contract does not have the correct balance"
    );
}

#[test]
fn new_register() {
    let test = EscrowTest::setup();

    // Add a proposal
    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens
    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    // Pick a escrow
    // The sginaturit ID is an UUID
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = amount_asked.mul(2);

    let balance_before_alice = test.token.balance(&test.alice);
    let balance_before_bob = test.token.balance(&test.bob);

    test.escrow
        .mock_auths(&[MockAuth {
            address: &test.bob,
            invoke: &MockAuthInvoke {
                contract: &test.token.address,
                fn_name: "transfer",
                args: (
                    test.bob.clone(),
                    test.escrow.address.clone(),
                    amount_to_give.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .register_escrow(&stocken_id, &signaturit_id, &test.bob, &amount_to_give);

    // The proposal owner should not receive the funds until the signature
    // process is completed
    assert_eq!(
        test.token.balance(&test.alice),
        balance_before_alice,
        "proposal owner balance has changed"
    );

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob - amount_to_give,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        amount_to_give,
        "escrow contract does not have the correct balance"
    );

    let event_expected = (
        test.escrow.address.clone(),
        (EscrowEvent2::RegisterEscrow(
            signaturit_id.clone(),
            stocken_id.clone(),
            0u32,
            test.bob.clone(),
            amount_to_give,
        )
        .name(),)
            .into_val(&test.env),
        (signaturit_id, stocken_id, 0u32, &test.bob, amount_to_give).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "register escrow event not present"
    );
}
