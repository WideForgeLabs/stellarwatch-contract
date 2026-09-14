#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};
use stellarwatch_shared::{ContractMetadata, Error};

mod storage;
pub mod test;

#[contract]
pub struct ContractRegistry;

#[contractimpl]
impl ContractRegistry {
    pub fn initialize(env: Env, owner: Address) -> Result<(), Error> {
        owner.require_auth();

        if storage::get_instance(&env).is_some() {
            return Err(Error::AlreadyInitialized);
        }

        let config = storage::RegistryConfig {
            owner,
            paused: false,
        };
        storage::set_instance(&env, &config);

        Ok(())
    }

    pub fn register_contract(
        env: Env,
        contract_id: BytesN<32>,
        name: String,
        version: String,
        deployer: Address,
    ) -> Result<(), Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        config.owner.require_auth();
        storage::extend_instance_ttl(&env);

        if storage::contract_exists(&env, &contract_id) {
            return Err(Error::InvalidInput);
        }

        let now = env.ledger().timestamp();
        let metadata = ContractMetadata {
            name: name.clone(),
            version,
            deployer: deployer.clone(),
            registered_at: now,
            last_check: None,
            active: true,
        };

        storage::set_contract(&env, &contract_id, &metadata);

        env.events().publish(
            ("registry", "registered"),
            (contract_id, name, deployer),
        );

        Ok(())
    }

    pub fn unregister_contract(env: Env, contract_id: BytesN<32>) -> Result<(), Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        config.owner.require_auth();
        storage::extend_instance_ttl(&env);

        if !storage::contract_exists(&env, &contract_id) {
            return Err(Error::ContractNotFound);
        }

        storage::remove_contract(&env, &contract_id);

        env.events().publish(
            ("registry", "unregistered"),
            (contract_id,),
        );

        Ok(())
    }

    pub fn is_registered(env: Env, contract_id: BytesN<32>) -> bool {
        storage::contract_exists(&env, &contract_id)
    }

    pub fn get_owner(env: Env) -> Result<Address, Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        Ok(config.owner)
    }

    pub fn is_paused(env: Env) -> Result<bool, Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        Ok(config.paused)
    }
}
