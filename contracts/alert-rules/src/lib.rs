#![no_std]

use soroban_sdk::{contract, contractimpl};

pub mod test;

#[contract]
pub struct AlertRules;

#[contractimpl]
impl AlertRules {
    pub fn ping() -> u32 {
        42
    }
}
