#![cfg(test)]

use soroban_sdk::Env;

use crate::{AlertRules, AlertRulesClient};

#[test]
fn test_ping() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AlertRules);
    let client = AlertRulesClient::new(&env, &contract_id);

    let result = client.ping();
    assert_eq!(result, 42);
}
