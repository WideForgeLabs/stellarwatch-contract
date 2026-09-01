#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

pub mod test;

#[contract]
pub struct HealthRegistry;

#[contractimpl]
impl HealthRegistry {
    pub fn initialize(env: Env, owner: Address) {
        owner.require_auth();
        env.storage().instance().set(&"owner", &owner);
    }

    pub fn get_owner(env: Env) -> Option<Address> {
        env.storage().instance().get(&"owner")
    }

    pub fn ping() -> u32 {
        42
    }
}
