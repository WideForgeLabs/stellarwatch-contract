#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

use crate::{ContractRegistry, ContractRegistryClient};

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, ContractRegistry);
    let client = ContractRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);

    client.initialize(&owner);

    let stored_owner = client.get_owner();
    assert_eq!(stored_owner, owner);
}

#[test]
fn test_initialize_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, ContractRegistry);
    let client = ContractRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);

    client.initialize(&owner);
    let result = client.try_initialize(&owner);
    assert!(result.is_err());
}

#[test]
fn test_register_and_unregister() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, ContractRegistry);
    let client = ContractRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);

    client.initialize(&owner);

    let target = BytesN::from_array(&env, &[9u8; 32]);
    let name = String::from_str(&env, "Test Contract");
    let version = String::from_str(&env, "v1.0.0");
    let deployer = Address::generate(&env);

    client.register_contract(&target, &name, &version, &deployer);
    assert!(client.is_registered(&target));

    client.unregister_contract(&target);
    assert!(!client.is_registered(&target));
}

#[test]
fn test_register_without_init_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, ContractRegistry);
    let client = ContractRegistryClient::new(&env, &contract_id);

    let target = BytesN::from_array(&env, &[9u8; 32]);
    let name = String::from_str(&env, "Test");
    let version = String::from_str(&env, "v1");
    let deployer = Address::generate(&env);

    let result = client.try_register_contract(&target, &name, &version, &deployer);
    assert!(result.is_err());
}
