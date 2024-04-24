#![no_std]
mod error;
mod events;
mod storage;
mod types;

use error::OracleError;
use events::OracleEvent;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, panic_with_error, vec, Address, Env, IntoVal, String, Symbol,
};
use storage::Storage;
use types::{DataKey, SignaturitProcess};

pub mod escrow {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/escrow.wasm");
    pub type EscrowClient<'a> = Client<'a>;
}
use escrow::EscrowClient;

fn check_initialization(env: &Env) {
    if !DataKey::Admin.has(env) {
        panic_with_error!(env, OracleError::NotInit);
    }
}

fn get_admin(env: &Env) -> Address {
    DataKey::Admin.get(env).unwrap()
}

#[contract]
pub struct SignaturitOracle;

#[contractimpl]
impl SignaturitOracle {
    pub fn get_admin(env: Env) -> Address {
        check_initialization(&env);
        get_admin(&env)
    }
    /**
    Initialize the contract with the given arguments, making the oracle ready
    to be used.

    ### Arguments
    * `admin`: The address that will be authorized to send the callback transactions.
    */
    pub fn initialize(env: Env, admin: Address) {
        // Check if Oracle is already initialized
        if DataKey::Admin.has(&env) {
            panic_with_error!(env, OracleError::AlreadyInit);
        }

        DataKey::Admin.set(&env, &admin);

        OracleEvent::Initialized(admin).publish(&env);
    }

    /**
    Register the signaturit ID (UUID) on the oracle to be observed and make
    callbacks based on their status.

    ### Arguments
    * `caller`: the address that register the `signaturit id` and where the
    callback response will be sent to.
    * `signaturit_id`: the UUID value obtained from signaturit to identify the
    signature process.
    */
    pub fn register_new_signature_process(env: Env, caller: Address, signaturit_id: String) -> u32 {
        check_initialization(&env);
        caller.require_auth();

        let oracle_id = DataKey::RegisterCounter.get(&env).unwrap_or(0);

        if DataKey::SignaturitProcess(oracle_id.clone()).has(&env) {
            panic_with_error!(env, OracleError::SignatureIdAlredyExist);
        }

        let signature_process = SignaturitProcess {
            id: oracle_id.clone(),
            signaturit_id: signaturit_id.clone(),
            send_to: caller,
        };

        OracleEvent::NewSignatureProcess(signaturit_id, oracle_id).publish(&env);

        DataKey::SignaturitProcess(oracle_id).set(&env, &signature_process);
        DataKey::RegisterCounter.set(&env, &(oracle_id + 1));

        return oracle_id;
    }

    pub fn signature_response(
        env: Env,
        oracle_id: u32,
        is_success: bool,
        document_hash: Option<String>,
    ) {
        check_initialization(&env);
        get_admin(&env).require_auth();

        let signature_process: SignaturitProcess = DataKey::SignaturitProcess(oracle_id.clone())
            .get(&env)
            .unwrap();

        let escrow_client = EscrowClient::new(&env, &signature_process.send_to);

        if is_success {
            // The signature proccess was completed (the stauts is `completed`)

            // Require the document hash
            if document_hash.is_none() {
                // Throw error
                panic_with_error!(env, OracleError::MissingDocHash);
            }

            // Grant auth to call `completed_signature``
            env.authorize_as_current_contract(vec![
                &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                        contract: signature_process.send_to,
                        fn_name: Symbol::new(&env, "completed_signature"),
                        args: (
                            signature_process.signaturit_id.clone(),
                            document_hash.clone().unwrap(),
                        )
                            .into_val(&env),
                    },
                    sub_invocations: vec![&env],
                }),
            ]);

            // Call the escrow
            escrow_client
                .completed_signature(&signature_process.signaturit_id, &document_hash.unwrap());
        } else {
            // The signature process has failed (the staus is expired, canceled or declined)

            env.authorize_as_current_contract(vec![
                &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                        contract: signature_process.send_to,
                        fn_name: Symbol::new(&env, "failed_signature"),
                        args: (signature_process.signaturit_id.clone(),).into_val(&env),
                    },
                    sub_invocations: vec![&env],
                }),
            ]);

            // Call failed
            escrow_client.failed_signature(&signature_process.signaturit_id);
        }

        OracleEvent::SignatureResponse(signature_process.signaturit_id, oracle_id, is_success)
            .publish(&env);
    }
}
