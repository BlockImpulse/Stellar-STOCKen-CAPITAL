use soroban_sdk::{symbol_short, Symbol};

/**
 * Proposal topic
 */
pub const PROPOSAL_TOPIC: Symbol = symbol_short!("PROPOSAL");

/**
 * Register topic
 */
pub const REGISTER_TOPIC: Symbol = symbol_short!("REGISTER");

/**
 * Initialization topic
 */
pub const INITIALIZED_TOPIC: Symbol = symbol_short!("INITIALZD");

/**
 * Signed topic
 */
pub const SIGNED_TOPIC: Symbol = symbol_short!("SIGNED");

/**
 * Completed topic
 */
pub const COMPLETED_TOPIC: Symbol = symbol_short!("COMPLETED");

/**
 * Failed topic symbol
 */
pub const FAILED_TOPIC: Symbol = symbol_short!("FAILED");

/**
 * Signature process completed topic
 */
pub const SIGNED_COMPLETED_TOPIC: (Symbol, Symbol) = (SIGNED_TOPIC, COMPLETED_TOPIC);

/**
 * Signature process failed topic
 */
pub const SIGNED_FAILED_TOPIC: (Symbol, Symbol) = (SIGNED_TOPIC, FAILED_TOPIC);
