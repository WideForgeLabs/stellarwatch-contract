use soroban_sdk::{contracttype, Address, BytesN, Env, Vec};
use stellarwatch_shared::AlertRule;

const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 17_280;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AlertConfig {
    pub registry: Address,
    pub health_registry: Address,
    pub owner: Address,
    pub paused: bool,
    pub next_rule_id: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Instance,
    Rule(u64),
    ContractRules(BytesN<32>),
}

pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn extend_persistent_ttl(env: &Env, key: &DataKey) {
    env.storage().persistent().extend_ttl(
        key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

pub fn get_instance(env: &Env) -> Option<AlertConfig> {
    env.storage().instance().get(&DataKey::Instance)
}

pub fn set_instance(env: &Env, config: &AlertConfig) {
    env.storage().instance().set(&DataKey::Instance, config);
    extend_instance_ttl(env);
}

pub fn get_rule(env: &Env, rule_id: u64) -> Option<AlertRule> {
    env.storage().persistent().get(&DataKey::Rule(rule_id))
}

pub fn set_rule(env: &Env, rule_id: u64, rule: &AlertRule) {
    let key = DataKey::Rule(rule_id);
    env.storage().persistent().set(&key, rule);
    extend_persistent_ttl(env, &key);
}

pub fn remove_rule(env: &Env, rule_id: u64) {
    env.storage().persistent().remove(&DataKey::Rule(rule_id));
}

pub fn get_contract_rules(env: &Env, contract_id: &BytesN<32>) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::ContractRules(contract_id.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn set_contract_rules(env: &Env, contract_id: &BytesN<32>, rules: &Vec<u64>) {
    let key = DataKey::ContractRules(contract_id.clone());
    env.storage().persistent().set(&key, rules);
    extend_persistent_ttl(env, &key);
}
