#![no_std]
use soroban_sdk::{contractclient, Env, String};

#[contractclient(name = "OracleConsumerClient")]
pub trait OracleConsumer {
    /**
    Return a succesful response for a given signature process. This means that the
    signature process was completed and the parites signed

    # Arguments

    * `signaturit_id` - The ID of the signature process that will be handled
    * `document_hash` - The hash of the document hat was signed
    */
    fn completed_signature(env: Env, signaturit_id: String, document_hash: String);

    /**
    Return a failed response for a given signature process. This means that the
    signature process was failed (parties declined, process was timeout, etc)
    */
    fn failed_signature(env: Env, signaturit_id: String);

    // TODO: Request for cancel signature process
    // It should ask to the oracle to cancel a specific signaturit ID.
    // The oracle store this request, emit an event and return an oracle_id for
    // this request. (finish the transacion here)
    //
    // Later, the oracle will wait for the listener (the cronjob) to check/cancel
    // the signature process using the signaturit API. Then the listener will send
    // a transaction to cancel the process (calling failed signature).
    //
    // There are two types of `oracle_id`. One for the SignaturitProcess and one
    // for CancelProcess. If the oracle_id does not exist on `SignaturitProcess`
    // then it is a `CancelProcess`.
    //
    // Also, update the function to receive as parameter an optional<DOC_HASH>
    // which will be the URI for the token
}
