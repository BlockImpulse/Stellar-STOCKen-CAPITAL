#![no_std]
use soroban_sdk::{contract, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    OnlyOwner = 1,
    OnlyOracle = 2,
}

#[contract]
pub struct EscrowContract;
