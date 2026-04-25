#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, Address, Env};

#[test]
fn test_1_happy_path() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Pamasahe);
    let client = PamasaheClient::new(&env, &contract_id);

    let commuter = Address::generate(&env);
    let driver = Address::generate(&env);
    let tx_id = symbol_short!("RIDE001");

    // Act
    client.pay_fare(&commuter, &driver, &15_0000000, &tx_id);

    // Assert
    assert_eq!(client.verify_payment(&tx_id), true);
}

#[test]
#[should_panic(expected = "Amount must be greater than zero")]
fn test_2_edge_case_zero_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Pamasahe);
    let client = PamasaheClient::new(&env, &contract_id);

    let commuter = Address::generate(&env);
    let driver = Address::generate(&env);

    // Trying to pay 0 should fail
    client.pay_fare(&commuter, &driver, &0, &symbol_short!("FAIL01"));
}

#[test]
fn test_3_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Pamasahe);
    let client = PamasaheClient::new(&env, &contract_id);

    let commuter = Address::generate(&env);
    let driver = Address::generate(&env);
    let amount = 20_0000000;
    let tx_id = symbol_short!("CHECK001");

    client.pay_fare(&commuter, &driver, &amount, &tx_id);

    // Read directly from storage
    let history: Map<Symbol, (Address, Address, i128)> = env
        .storage()
        .persistent()
        .get(&StorageKey::TransactionHistory)
        .unwrap();

    let (stored_from, stored_to, stored_amt) = history.get(tx_id).unwrap();

    assert_eq!(stored_from, commuter);
    assert_eq!(stored_to, driver);
    assert_eq!(stored_amt, amount);
}

#[test]
#[should_panic(expected = "Cannot pay to yourself")]
fn test_4_same_address() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Pamasahe);
    let client = PamasaheClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.pay_fare(&user, &user, &10_0000000, &symbol_short!("SELF01"));
}

#[test]
fn test_5_get_transaction_details() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Pamasahe);
    let client = PamasaheClient::new(&env, &contract_id);

    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let tx_id = symbol_short!("DETAILS");

    client.pay_fare(&a, &b, &12_5000000, &tx_id);

    let details = client.get_transaction(&tx_id);
    assert_eq!(details.0, a);
    assert_eq!(details.1, b);
    assert_eq!(details.2, 12_5000000);
}
