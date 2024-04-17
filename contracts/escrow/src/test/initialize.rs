#![cfg(test)]

use crate::{
    events::INITIALIZED_TOPIC,
    test::{escrow::EscrowError, EscrowTest},
};
use soroban_sdk::{testutils::Events, IntoVal};

#[test]
fn before_initialization() {
    let test = EscrowTest::setup_non_init();

    let res_get_oracle = test.escrow.try_get_oracle();
    let res_get_asset = test.escrow.try_get_asset();

    assert_eq!(res_get_oracle, Err(Ok(EscrowError::NotInit.into())));
    assert_eq!(res_get_asset, Err(Ok(EscrowError::NotInit.into())));
}

#[test]
fn initialization() {
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

#[test]
fn double_initialization() {
    let test = EscrowTest::setup_non_init();

    // First initialization
    test.escrow
        .initialize(&test.token.address, &test.oracle.address);

    let res = test
        .escrow
        .try_initialize(&test.token.address, &test.oracle.address);

    assert_eq!(res, Err(Ok(EscrowError::AlreadyInitialized.into())));
}
