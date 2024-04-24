#![cfg(test)]

use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, Symbol};

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

pub mod escrow_test {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/escrow_test.wasm"
    );
    pub type EscrowClient<'a> = Client<'a>;
}
use escrow_test::EscrowClient;

fn create_escrow_contract<'a>(env: &Env) -> EscrowClient<'a> {
    let contract_id = env.register_contract_wasm(None, escrow_test::WASM);
    EscrowClient::new(&env, &contract_id)
}

// These topics are from EscrowTest
pub const COMPLETED_TOPIC: Symbol = symbol_short!("COMP_TEST");
pub const FAILED_TOPIC: Symbol = symbol_short!("FAIL_TEST");

pub struct OracleTest<'a> {
    env: Env,
    oracle: OracleClient<'a>,
    escrow: EscrowClient<'a>,
    alice: Address,
    bob: Address,
    admin: Address,
}

impl<'a> OracleTest<'a> {
    fn setup() -> Self {
        let test_setup = Self::setup_non_init();

        // Initialize contracts
        // ORACLE
        test_setup.oracle.initialize(&test_setup.admin);

        // ESCROW
        test_setup.escrow.initialize(&test_setup.oracle.address);

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

        return OracleTest {
            env,
            oracle: oracle_client,
            escrow: escrow_client,
            alice,
            bob,
            admin,
        };
    }
}

mod initialize;
mod register;
mod signature_response;
