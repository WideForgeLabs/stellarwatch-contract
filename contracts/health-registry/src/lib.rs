#![no_std]
use soroban_sdk::contractmeta;

contractmeta!(
    key = "source_repo",
    val = "github:WideForgeLabs/stellarwatch-contract#befd768c9d2a0f0925357c10f49eacac92fab56f"
);

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};
use stellarwatch_shared::{Error, HealthRecord, HealthStatus};

mod storage;
pub mod test;

#[contract]
pub struct HealthRegistry;

#[contractimpl]
impl HealthRegistry {
    pub fn initialize(env: Env, owner: Address, registry: Address) -> Result<(), Error> {
        owner.require_auth();

        if storage::get_instance(&env).is_some() {
            return Err(Error::AlreadyInitialized);
        }

        let config = storage::HealthConfig {
            registry,
            owner,
            max_history: 1000,
        };
        storage::set_instance(&env, &config);

        Ok(())
    }

    pub fn record_health(
        env: Env,
        contract_id: BytesN<32>,
        status: HealthStatus,
        response_time_ms: Option<u32>,
        message: String,
    ) -> Result<(), Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        storage::extend_instance_ttl(&env);

        let counter = storage::get_counter(&env, &contract_id);
        if counter >= config.max_history as u64 {
            return Err(Error::MaxHistoryExceeded);
        }

        let new_index = counter + 1;
        let now = env.ledger().timestamp();
        let checker = env.current_contract_address();

        let record = HealthRecord {
            timestamp: now,
            checker: checker.clone(),
            status,
            response_time_ms,
            message: message.clone(),
            block_height: env.ledger().sequence(),
        };

        storage::set_record(&env, &contract_id, new_index, &record);
        storage::set_counter(&env, &contract_id, new_index);

        env.events()
            .publish(("health", "recorded"), (contract_id, now, status, message));

        Ok(())
    }

    pub fn get_health_history(env: Env, contract_id: BytesN<32>, limit: u32) -> Vec<HealthRecord> {
        let counter = storage::get_counter(&env, &contract_id);
        let mut records = Vec::new(&env);

        if counter == 0 {
            return records;
        }

        let take = if (limit as u64) < counter {
            limit as u64
        } else {
            counter
        };

        let start = counter - take + 1;
        let mut i = counter;

        while i >= start {
            if let Some(record) = storage::get_record(&env, &contract_id, i) {
                records.push_back(record);
            }
            if i == start {
                break;
            }
            i -= 1;
        }

        records
    }

    pub fn get_latest_health(env: Env, contract_id: BytesN<32>) -> Option<HealthRecord> {
        let counter = storage::get_counter(&env, &contract_id);
        if counter == 0 {
            return None;
        }
        storage::get_record(&env, &contract_id, counter)
    }

    pub fn get_owner(env: Env) -> Result<Address, Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        Ok(config.owner)
    }

    pub fn get_registry(env: Env) -> Result<Address, Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        Ok(config.registry)
    }
}
