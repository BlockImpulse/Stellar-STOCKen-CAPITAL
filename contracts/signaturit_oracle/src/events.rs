use soroban_sdk::{symbol_short, Symbol};

/**
Register topic
*/
const REGISTER_TOPIC: Symbol = symbol_short!("REGISTER");

/**
New topic
*/
const NEW_TOPIC: Symbol = symbol_short!("NEW");

/**
Signature topic
*/
const SIGNATURE_TOPIC: Symbol = symbol_short!("SIGNATURE");

/**
Response topic
*/
const RESPONSE_TOPIC: Symbol = symbol_short!("RESPONSE");

/**
Topic for the event when a new signature process is registered
*/
pub const REGISTER_NEW_SIGNATURE_TOPIC: (Symbol, Symbol, Symbol) =
    (REGISTER_TOPIC, NEW_TOPIC, SIGNATURE_TOPIC);

/**
Topic for the event when the response for a Signature process is returned
*/
pub const SIGNATURE_RESPONSE_TOPIC: (Symbol, Symbol) = (SIGNATURE_TOPIC, RESPONSE_TOPIC);
