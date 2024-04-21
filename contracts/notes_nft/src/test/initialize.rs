#![cfg(test)]

use crate::{
    events::INITIALIZED_TOPIC,
    test::{Error, NotesNFTTest},
};
use soroban_sdk::{testutils::Events, IntoVal, String};

#[test]
fn initialization() {
    let test = NotesNFTTest::setup_non_init();

    let name = String::from_str(&test.env, "Test NFT Name");
    let symbol = String::from_str(&test.env, "TEST_SYMBOL");

    test.notes_nft
        .initialize(&test.admin_escrow, &name, &symbol);

    let event_expected = (
        test.notes_nft.address.clone(),
        (INITIALIZED_TOPIC,).into_val(&test.env),
        (test.admin_escrow, name, symbol).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "initialized event not present"
    );
}

#[test]
fn double_initialization() {
    let test = NotesNFTTest::setup_non_init();

    let name = String::from_str(&test.env, "Test NFT Name");
    let symbol = String::from_str(&test.env, "TEST_SYMBOL");

    // First initialization
    test.notes_nft
        .initialize(&test.admin_escrow, &name, &symbol);

    let res = test
        .notes_nft
        .try_initialize(&test.admin_escrow, &name, &symbol);

    assert_eq!(res, Err(Ok(Error::AlreadyInit.into())));
}
