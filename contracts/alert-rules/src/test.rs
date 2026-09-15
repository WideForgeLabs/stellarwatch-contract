#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

use crate::{AlertRules, AlertRulesClient};
use stellarwatch_shared::{AlertCondition, Severity};

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AlertRules);
    let client = AlertRulesClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);
    let health = Address::generate(&env);

    client.initialize(&owner, &registry, &health);

    assert_eq!(client.get_owner(), owner);
}

#[test]
fn test_create_and_get_rule() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AlertRules);
    let client = AlertRulesClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);
    let health = Address::generate(&env);

    client.initialize(&owner, &registry, &health);

    let target = BytesN::from_array(&env, &[5u8; 32]);
    let notif = String::from_str(&env, "webhook://example");

    let rule_id = client.create_rule(
        &target,
        &AlertCondition::ResponseTime,
        &Severity::Warn,
        &500,
        &notif,
    );

    assert_eq!(rule_id, 1);

    let rule = client.get_rule(&rule_id).unwrap();
    assert_eq!(rule.threshold_value, 500);
    assert_eq!(rule.severity, Severity::Warn);
    assert!(rule.active);
}

#[test]
fn test_delete_rule() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AlertRules);
    let client = AlertRulesClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);
    let health = Address::generate(&env);

    client.initialize(&owner, &registry, &health);

    let target = BytesN::from_array(&env, &[6u8; 32]);
    let notif = String::from_str(&env, "webhook://example");

    let rule_id = client.create_rule(
        &target,
        &AlertCondition::StatusChange,
        &Severity::Critical,
        &0,
        &notif,
    );

    client.delete_rule(&rule_id);

    let result = client.get_rule(&rule_id);
    assert!(result.is_none());
}

#[test]
fn test_pause_and_resume() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AlertRules);
    let client = AlertRulesClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let registry = Address::generate(&env);
    let health = Address::generate(&env);

    client.initialize(&owner, &registry, &health);

    assert!(!client.is_paused());
    client.pause_alerts();
    assert!(client.is_paused());
    client.resume_alerts();
    assert!(!client.is_paused());
}

#[test]
fn test_create_without_init_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AlertRules);
    let client = AlertRulesClient::new(&env, &contract_id);

    let target = BytesN::from_array(&env, &[7u8; 32]);
    let notif = String::from_str(&env, "webhook://example");

    let result = client.try_create_rule(
        &target,
        &AlertCondition::ResponseTime,
        &Severity::Info,
        &100,
        &notif,
    );
    assert!(result.is_err());
}
