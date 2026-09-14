use soroban_sdk::{contracttype, Address, BytesN, Env};
use stellarwatch_shared::ContractMetadata;

// TTL constants: ~1 day threshold, ~30 days bump at ~5 sec/ledger
const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 17_280;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RegistryConfig {
    pub owner: Address,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Instance,
    Contract(BytesN<32>),
}

pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn extend_persistent_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn get_instance(env: &Env) -> Option<RegistryConfig> {
    env.storage().instance().get(&DataKey::Instance)
}

pub fn set_instance(env: &Env, config: &RegistryConfig) {
    env.storage().instance().set(&DataKey::Instance, config);
    extend_instance_ttl(env);
}

pub fn contract_exists(env: &Env, contract_id: &BytesN<32>) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Contract(contract_id.clone()))
}

pub fn set_contract(env: &Env, contract_id: &BytesN<32>, metadata: &ContractMetadata) {
    let key = DataKey::Contract(contract_id.clone());
    env.storage().persistent().set(&key, metadata);
    extend_persistent_ttl(env, &key);
}

pub fn get_contract(env: &Env, contract_id: &BytesN<32>) -> Option<ContractMetadata> {
    env.storage()
        .persistent()
        .get(&DataKey::Contract(contract_id.clone()))
}

pub fn remove_contract(env: &Env, contract_id: &BytesN<32>) {
    env.storage()
        .persistent()
        .remove(&DataKey::Contract(contract_id.clone()));
}
