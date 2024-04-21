#![cfg(test)]
extern crate std;

use super::{Error, NotesNFTContract, NotesNFTContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

// pub mod notes_nft {
//     soroban_sdk::contractimport!(
//         file = "../../target/wasm32-unknown-unknown/release/notes_nft.wasm"
//     );
//     pub type NotesNFTClient<'a> = Client<'a>;
// }
// use notes_nft::NotesNFTClient;

fn create_notes_nft_contract<'a>(env: &Env) -> NotesNFTContractClient<'a> {
    let contract_id = env.register_contract(None, NotesNFTContract);
    let contract_client = NotesNFTContractClient::new(env, &contract_id);
    // let contract_id = env.register_contract_wasm(None, notes_nft::WASM);
    // let contract_client = NotesNFTClient::new(env, &contract_id);
    contract_client
}

pub struct NotesNFTTest<'a> {
    env: Env,
    notes_nft: NotesNFTContractClient<'a>,
    admin_escrow: Address,
    alice: Address,
    bob: Address,
}

impl<'a> NotesNFTTest<'a> {
    fn setup_non_init() -> Self {
        let env = Env::default();

        // Generate the accounts (users)
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let admin_escrow = Address::generate(&env);
        assert_ne!(alice, bob);
        assert_ne!(alice, admin_escrow);
        assert_ne!(bob, admin_escrow);

        // Create the contract
        let notes_nft_client = create_notes_nft_contract(&env);

        return NotesNFTTest {
            env,
            notes_nft: notes_nft_client,
            admin_escrow,
            alice,
            bob,
        };
    }
    fn setup() -> Self {
        let test = NotesNFTTest::setup_non_init();

        let name = String::from_str(&test.env, "Signaturit Notes NFT");
        let symbol = String::from_str(&test.env, "SN_NFT");

        test.notes_nft
            .initialize(&test.admin_escrow, &name, &symbol);

        return test;
    }
}

#[test]
fn pave_xda() {
    let test = NotesNFTTest::setup();

    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);
    let doc_hash_2 = String::from_bytes(&test.env, &[1; 32]);

    test.notes_nft
        .mock_all_auths()
        .mint(&test.alice, &doc_hash_1);
    assert_eq!(test.notes_nft.balance_of(&test.alice), 1);
}

pub mod initialize;
pub mod mint;
