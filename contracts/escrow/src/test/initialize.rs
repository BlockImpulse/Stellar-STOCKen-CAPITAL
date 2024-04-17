#![cfg(test)]

extern crate std;

use crate::{events::INITIALIZED_TOPIC, test::EscrowTest};
use soroban_sdk::{testutils::Events, IntoVal};

#[test]
fn test_escrow_initialization() {
    let test = EscrowTest::setup_non_init();

    test.escrow
        .initialize(&test.token.address, &test.oracle.address);

    let event_expected = (
        test.escrow.address.clone(),
        (INITIALIZED_TOPIC,).into_val(&test.env),
        (test.token.address.clone(), test.oracle.address.clone()).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "initialized event not present"
    );
}
