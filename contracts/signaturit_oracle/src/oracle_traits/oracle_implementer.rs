use soroban_sdk::{contractclient, Env, String};

/// Averr
#[contractclient(name = "OracleImplementerClient")]
pub trait OracleImplementerInterface {
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
}
