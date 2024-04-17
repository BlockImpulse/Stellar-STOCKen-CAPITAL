#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

pub mod oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/signaturit_oracle.wasm"
    );
    pub type OracleClient<'a> = Client<'a>;
}

pub mod escrow {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/escrow.wasm");
    pub type EscrowClient<'a> = Client<'a>;
}

fn create_escrow_contract<'a>(env: &Env) -> escrow::EscrowClient<'a> {
    let contract_id = env.register_contract_wasm(None, escrow::WASM);
    escrow::EscrowClient::new(&env, &contract_id)
}

fn create_oracle_contract<'a>(env: &Env) -> oracle::OracleClient<'a> {
    let contract_id = env.register_contract_wasm(None, oracle::WASM);
    oracle::OracleClient::new(env, &contract_id)
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

pub struct EscrowTest<'a> {
    env: Env,
    escrow: escrow::EscrowClient<'a>,
    oracle: oracle::OracleClient<'a>,
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

        test_setup
            .escrow
            .initialize(&test_setup.token.address, &test_setup.oracle.address);

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
            token,
            alice,
            bob,
            admin,
        };
    }
}

pub mod add_proposal;
pub mod initialize;
pub mod register;
