#![no_std]

pub mod oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/signaturit_oracle.wasm"
    );
    pub type OracleClient<'a> = Client<'a>;
}

use oracle_traits::OracleConsumer;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, symbol_short, vec, Address, Env, IntoVal, String, Symbol,
};

pub enum EscrowTestEvent {
    Initialized(Address, Address, Address),
    NewProposal(String, Address),
    RegisterEscrow(String, String, u32, Address, i128),
    SignedCompleted(String, String, Address, Address, i128, u32),
    SignedFailed(String, String, Address),
}

pub const COMPLETED_TOPIC: Symbol = symbol_short!("COMP_TEST");
pub const FAILED_TOPIC: Symbol = symbol_short!("FAIL_TEST");
const ORACLE: Symbol = symbol_short!("ORACLE");

fn get_oracle(env: &Env) -> Address {
    env.storage().persistent().get(&ORACLE).unwrap()
}

#[contract]
pub struct EscrowTest;

#[contractimpl]
impl EscrowTest {
    pub fn initialize(env: Env, oracle_address: Address) {
        env.storage().persistent().set(&ORACLE, &oracle_address);
        env.storage().persistent().extend_ttl(&ORACLE, 1000, 1000);
    }

    pub fn oracle_register(env: Env, signaturit_id: String) -> u32 {
        // Get the oracle client
        let oracle_client = oracle::OracleClient::new(&env, &get_oracle(&env));

        // Grant auth for calling the function
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: get_oracle(&env),
                    fn_name: Symbol::new(&env, "register_new_signature_process"),
                    args: (env.current_contract_address(), signaturit_id.clone()).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Get the oracle id for this process signature
        let oracle_id = oracle_client
            .register_new_signature_process(&env.current_contract_address(), &signaturit_id);

        return oracle_id;
    }
}

#[contractimpl]
impl OracleConsumer for EscrowTest {
    fn completed_signature(env: Env, signaturit_id: String, document_hash: String) {
        env.events()
            .publish((COMPLETED_TOPIC,), (signaturit_id, document_hash));
    }

    fn failed_signature(env: Env, signaturit_id: String) {
        env.events().publish((FAILED_TOPIC,), signaturit_id);
    }
}
