#![cfg(test)]

use std::println;

use super::{EscrowContract, EscrowContractClient};
use soroban_sdk::{
    testutils::{BytesN, Logs},
    Bytes, Env,
};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(&env, &contract_id);

    let id = <soroban_sdk::BytesN<32>>::random(&env);

    client.add_proposal(&id);

    let logs = env.logs().all();
    std::println!("{}", logs.join("\n"));

    // let a: BytesN<32> = BytesN::<32>::random(&env);
}
