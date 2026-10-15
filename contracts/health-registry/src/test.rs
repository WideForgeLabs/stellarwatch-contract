#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

use crate::{HealthRegistry, HealthRegistryClient};
use stellarwatch_shared::HealthStatus;

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HealthRegistry);
    let client = HealthRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);

    client.initialize(&owner, &registry);

    let stored_registry = client.get_registry();
    assert_eq!(stored_registry, registry);
    assert_eq!(client.get_owner(), owner);
}

#[test]
fn test_record_health() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HealthRegistry);
    let client = HealthRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);

    client.initialize(&owner, &registry);

    let target = BytesN::from_array(&env, &[7u8; 32]);
    let msg = String::from_str(&env, "all good");

    client.record_health(&target, &HealthStatus::Healthy, &Some(120), &msg);

    let latest = client.get_latest_health(&target).unwrap();
    assert_eq!(latest.status, HealthStatus::Healthy);
    assert_eq!(latest.response_time_ms, Some(120));
}

#[test]
fn test_health_history_ordering() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HealthRegistry);
    let client = HealthRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);

    client.initialize(&owner, &registry);

    let target = BytesN::from_array(&env, &[8u8; 32]);
    let msg = String::from_str(&env, "ok");

    client.record_health(&target, &HealthStatus::Healthy, &Some(100), &msg);
    client.record_health(&target, &HealthStatus::Degraded, &Some(300), &msg);
    client.record_health(&target, &HealthStatus::Unhealthy, &Some(900), &msg);

    let history = client.get_health_history(&target, &10);
    assert_eq!(history.len(), 3);
    assert_eq!(history.get(0).unwrap().status, HealthStatus::Unhealthy);
    assert_eq!(history.get(2).unwrap().status, HealthStatus::Healthy);
}

#[test]
fn test_record_without_init_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HealthRegistry);
    let client = HealthRegistryClient::new(&env, &contract_id);

    let target = BytesN::from_array(&env, &[9u8; 32]);
    let msg = String::from_str(&env, "test");

    let result = client.try_record_health(&target, &HealthStatus::Healthy, &None, &msg);
    assert!(result.is_err());
}

#[test]
fn test_empty_history() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HealthRegistry);
    let client = HealthRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);

    client.initialize(&owner, &registry);

    let target = BytesN::from_array(&env, &[10u8; 32]);
    let history = client.get_health_history(&target, &10);
    assert_eq!(history.len(), 0);
}

#[test]
fn test_prune_history() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HealthRegistry);
    let client = HealthRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);

    client.initialize(&owner, &registry);

    let target = BytesN::from_array(&env, &[11u8; 32]);
    let msg = String::from_str(&env, "ok");

    // Record 5 health entries
    client.record_health(&target, &HealthStatus::Healthy, &Some(100), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(110), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(120), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(130), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(140), &msg);

    // Confirm 5 records exist
    let before = client.get_health_history(&target, &10);
    assert_eq!(before.len(), 5);

    // Prune down to 2
    client.prune_history(&target, &2);

    // Verify only 2 records remain
    let after = client.get_health_history(&target, &10);
    assert_eq!(after.len(), 2);
}

#[test]
fn test_auto_prune_on_max_history() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HealthRegistry);
    let client = HealthRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);

    client.initialize(&owner, &registry);

    // Set max_history low so we hit the limit quickly
    client.set_max_history(&3);

    let target = BytesN::from_array(&env, &[12u8; 32]);
    let msg = String::from_str(&env, "ok");

    // Record 5 entries; the first 2 should be pruned automatically
    client.record_health(&target, &HealthStatus::Healthy, &Some(100), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(110), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(120), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(130), &msg);
    client.record_health(&target, &HealthStatus::Healthy, &Some(140), &msg);

    let history = client.get_health_history(&target, &10);
    assert_eq!(history.len(), 3);

    // Newest should be the last one recorded
    let latest = history.get(0).unwrap();
    assert_eq!(latest.response_time_ms, Some(140));
}
