#![cfg(test)]

use crate::{
    events::EscrowEvent,
    test::{escrow::EscrowError, EscrowTest},
};
use soroban_sdk::{testutils::Events, IntoVal};

#[test]
fn before_initialization() {
    let test = EscrowTest::setup_non_init();

    let res_get_oracle = test.escrow.try_get_oracle();
    let res_get_asset = test.escrow.try_get_asset();
    let res_get_nft_notes = test.escrow.try_get_nft_notes();

    assert_eq!(res_get_oracle, Err(Ok(EscrowError::NotInit.into())));
    assert_eq!(res_get_asset, Err(Ok(EscrowError::NotInit.into())));
    assert_eq!(res_get_nft_notes, Err(Ok(EscrowError::NotInit.into())));
}

#[test]
fn initialization_xd() {
    let test = EscrowTest::setup_non_init();

    test.escrow.initialize(
        &test.token.address,
        &test.oracle.address,
        &test.nft_notes.address,
    );

    // Check Initialized event
    let event_expected = (
        test.escrow.address.clone(),
        (EscrowEvent::Initialized(
            test.token.address.clone(),
            test.oracle.address.clone(),
            test.nft_notes.address.clone(),
        )
        .name(),)
            .into_val(&test.env),
        (
            &test.token.address,
            &test.oracle.address,
            &test.nft_notes.address,
        )
            .into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "initialized event not present"
    );

    assert_eq!(test.escrow.get_asset(), test.token.address);
    assert_eq!(test.escrow.get_oracle(), test.oracle.address);
    assert_eq!(test.escrow.get_nft_notes(), test.nft_notes.address);
}

#[test]
fn double_initialization() {
    let test = EscrowTest::setup_non_init();

    // First initialization
    test.escrow.initialize(
        &test.token.address,
        &test.oracle.address,
        &test.nft_notes.address,
    );

    let res = test.escrow.try_initialize(
        &test.token.address,
        &test.oracle.address,
        &test.nft_notes.address,
    );

    assert_eq!(res, Err(Ok(EscrowError::AlreadyInitialized.into())));
}
