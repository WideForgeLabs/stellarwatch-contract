use soroban_sdk::{contracttype, Address, BytesN, Env};
use stellarwatch_shared::{HealthRecord, HealthStatus};

// TTL constants: ~1 day threshold, ~30 days bump at ~5 sec/ledger
const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 17_280;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct HealthConfig {
    pub registry: Address,
    pub owner: Address,
    pub max_history: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Instance,
    Counter(BytesN<32>),
    Record(BytesN<32>, u64),
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

// Instance storage
pub fn get_instance(env: &Env) -> Option<HealthConfig> {
    env.storage().instance().get(&DataKey::Instance)
}

pub fn set_instance(env: &Env, config: &HealthConfig) {
    env.storage().instance().set(&DataKey::Instance, config);
    extend_instance_ttl(env);
}

// Counter (per contract)
pub fn get_counter(env: &Env, contract_id: &BytesN<32>) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::Counter(contract_id.clone()))
        .unwrap_or(0)
}

pub fn set_counter(env: &Env, contract_id: &BytesN<32>, value: u64) {
    let key = DataKey::Counter(contract_id.clone());
    env.storage().persistent().set(&key, &value);
    extend_persistent_ttl(env, &key);
}

// Records
pub fn get_record(env: &Env, contract_id: &BytesN<32>, index: u64) -> Option<HealthRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::Record(contract_id.clone(), index))
}

pub fn set_record(env: &Env, contract_id: &BytesN<32>, index: u64, record: &HealthRecord) {
    let key = DataKey::Record(contract_id.clone(), index);
    env.storage().persistent().set(&key, record);
    extend_persistent_ttl(env, &key);
}

#[allow(dead_code)]
pub fn remove_record(env: &Env, contract_id: &BytesN<32>, index: u64) {
    env.storage()
        .persistent()
        .remove(&DataKey::Record(contract_id.clone(), index));
}

// Marker to silence "unused" warning for HealthStatus import used in lib.rs
pub fn _use_health_status(_s: HealthStatus) {}
