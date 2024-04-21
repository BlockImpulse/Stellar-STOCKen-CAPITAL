#![cfg(test)]

use crate::{
    events::INITIALIZED_TOPIC,
    test::{Error, NotesNFTTest},
};
use soroban_sdk::{testutils::Events, IntoVal, String};

#[test]
fn mint() {
    let test = NotesNFTTest::setup();
}
