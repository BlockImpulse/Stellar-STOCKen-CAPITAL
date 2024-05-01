#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, String,
};

pub mod oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/signaturit_oracle.wasm"
    );
    pub type OracleClient<'a> = Client<'a>;
}
use oracle::OracleClient;

fn create_oracle_contract<'a>(env: &Env) -> OracleClient<'a> {
    let contract_id = env.register_contract_wasm(None, oracle::WASM);
    let contract_client = OracleClient::new(env, &contract_id);
    contract_client
}

pub mod escrow {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/escrow.wasm");
    pub type EscrowClient<'a> = Client<'a>;
}
use escrow::EscrowClient;

fn create_escrow_contract<'a>(env: &Env) -> EscrowClient<'a> {
    let contract_id = env.register_contract_wasm(None, escrow::WASM);
    EscrowClient::new(&env, &contract_id)
}

pub mod notes_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/notes_nft.wasm"
    );
    pub type NotesNFTClient<'a> = Client<'a>;
}
use notes_nft::NotesNFTClient;

fn create_nft_contract<'a>(env: &Env) -> NotesNFTClient<'a> {
    let contract_id = env.register_contract_wasm(None, notes_nft::WASM);
    NotesNFTClient::new(&env, &contract_id)
}

fn create_token_contract<'a>(
    env: &Env,
    admin: &Address,
) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let contract_address = env.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(env, &contract_address),
        StellarAssetClient::new(env, &contract_address),
    )
}

// keccak256(STOCKEN_ID_1)
pub const STOCKEN_ID_1: &str = "6ef7e237bbddb133bb3504cad9e2ec7ff90c0c9b63567a632dbad8bb2b923728";
// keccak256(STOCKEN_ID_2)
pub const STOCKEN_ID_2: &str = "af8f0b8ba4749d7a83edcd03a18e3ee3807fca630f8a18e8e59be53ea15c9e95";

pub struct EscrowTest<'a> {
    env: Env,
    escrow: EscrowClient<'a>,
    oracle: OracleClient<'a>,
    nft_notes: NotesNFTClient<'a>,
    token: TokenClient<'a>,
    alice: Address,
    bob: Address,
    admin: Address,
}

impl<'a> EscrowTest<'a> {
    fn setup() -> Self {
        let test_setup = Self::setup_non_init();

        // Initialize both contracts
        test_setup.oracle.initialize(&test_setup.admin);

        test_setup.escrow.initialize(
            &test_setup.token.address,
            &test_setup.oracle.address,
            &test_setup.nft_notes.address,
        );

        let name = String::from_str(&test_setup.env, "Signaturit Notes NFT");
        let symbol = String::from_str(&test_setup.env, "SN_NFT");

        test_setup
            .nft_notes
            .initialize(&test_setup.escrow.address, &name, &symbol);

        return test_setup;
    }

    fn setup_non_init() -> Self {
        let env = Env::default();

        // Generate the accounts (users)
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let admin = Address::generate(&env);
        assert_ne!(alice, bob);
        assert_ne!(alice, admin);
        assert_ne!(bob, admin);

        // Create the contracts
        let escrow_client = create_escrow_contract(&env);
        let oracle_client = create_oracle_contract(&env);
        let nft_client = create_nft_contract(&env);

        let (token, token_admin) = create_token_contract(&env, &admin);

        let amount: i128 = 1_000_000_000_000_000_000_000; // 1K tokens (18 decimals)

        // Mint tokens for both addresses with both tokens
        env.mock_all_auths();
        token_admin.mint(&alice, &amount);
        token_admin.mint(&bob, &amount);

        return EscrowTest {
            env,
            escrow: escrow_client,
            oracle: oracle_client,
            nft_notes: nft_client,
            token,
            alice,
            bob,
            admin,
        };
    }
}

mod add_proposal;
mod initialize;
mod register;
mod signatures_response;
