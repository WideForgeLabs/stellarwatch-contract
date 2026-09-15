#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};
use stellarwatch_shared::{AlertCondition, AlertRule, Error, Severity};

mod storage;
pub mod test;

#[contract]
pub struct AlertRules;

#[contractimpl]
impl AlertRules {
    pub fn initialize(
        env: Env,
        owner: Address,
        registry: Address,
        health_registry: Address,
    ) -> Result<(), Error> {
        owner.require_auth();

        if storage::get_instance(&env).is_some() {
            return Err(Error::AlreadyInitialized);
        }

        let config = storage::AlertConfig {
            registry,
            health_registry,
            owner,
            paused: false,
            next_rule_id: 1,
        };
        storage::set_instance(&env, &config);

        Ok(())
    }

    pub fn create_rule(
        env: Env,
        contract_id: BytesN<32>,
        condition: AlertCondition,
        severity: Severity,
        threshold_value: u64,
        notification_target: String,
    ) -> Result<u64, Error> {
        let mut config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        config.owner.require_auth();
        storage::extend_instance_ttl(&env);

        let rule_id = config.next_rule_id;
        let now = env.ledger().timestamp();

        let rule = AlertRule {
            id: rule_id,
            contract_id: contract_id.clone(),
            condition,
            severity,
            threshold_value,
            active: true,
            created_at: now,
            updated_at: now,
            notification_target,
        };

        storage::set_rule(&env, rule_id, &rule);

        let mut rules = storage::get_contract_rules(&env, &contract_id);
        rules.push_back(rule_id);
        storage::set_contract_rules(&env, &contract_id, &rules);

        config.next_rule_id = rule_id + 1;
        storage::set_instance(&env, &config);

        env.events()
            .publish(("alert", "rule_created"), (rule_id, contract_id, severity));

        Ok(rule_id)
    }

    pub fn delete_rule(env: Env, rule_id: u64) -> Result<(), Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        config.owner.require_auth();
        storage::extend_instance_ttl(&env);

        let rule = storage::get_rule(&env, rule_id).ok_or(Error::RuleNotFound)?;

        storage::remove_rule(&env, rule_id);

        let rules = storage::get_contract_rules(&env, &rule.contract_id);
        let mut new_rules = Vec::new(&env);
        for r in rules.iter() {
            if r != rule_id {
                new_rules.push_back(r);
            }
        }
        storage::set_contract_rules(&env, &rule.contract_id, &new_rules);

        env.events().publish(("alert", "rule_deleted"), (rule_id,));

        Ok(())
    }

    pub fn get_rule(env: Env, rule_id: u64) -> Option<AlertRule> {
        storage::get_rule(&env, rule_id)
    }

    pub fn get_contract_rules(env: Env, contract_id: BytesN<32>) -> Vec<u64> {
        storage::get_contract_rules(&env, &contract_id)
    }

    pub fn pause_alerts(env: Env) -> Result<(), Error> {
        let mut config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        config.owner.require_auth();
        config.paused = true;
        storage::set_instance(&env, &config);

        env.events().publish(("alert", "paused"), ());
        Ok(())
    }

    pub fn resume_alerts(env: Env) -> Result<(), Error> {
        let mut config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        config.owner.require_auth();
        config.paused = false;
        storage::set_instance(&env, &config);

        env.events().publish(("alert", "resumed"), ());
        Ok(())
    }

    pub fn is_paused(env: Env) -> Result<bool, Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        Ok(config.paused)
    }

    pub fn get_owner(env: Env) -> Result<Address, Error> {
        let config = storage::get_instance(&env).ok_or(Error::NotInitialized)?;
        Ok(config.owner)
    }
}
