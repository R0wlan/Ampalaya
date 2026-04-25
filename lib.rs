#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Address, Env, Map, Symbol};

// Storage key for transaction history
#[repr(u32)]
enum StorageKey {
    TransactionHistory, // Map<TxID, (From, To, Amount)>
}

#[contract]
pub struct Pamasahe;

#[contractimpl]
impl Pamasahe {
    /// Initialize the contract
    pub fn __constructor(_env: Env) {}

    /// Pay fare from commuter to driver.
    /// Records the transaction on-chain for full transparency.
    pub fn pay_fare(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        tx_ref: Symbol,
    ) {
        // Ensure the sender authorizes this transaction
        from.require_auth();

        // Basic validation
        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }
        if from == to {
            panic!("Cannot pay to yourself");
        }

        // Store transaction details in history
        let mut history: Map<Symbol, (Address, Address, i128)> = env
            .storage()
            .persistent()
            .get(&StorageKey::TransactionHistory)
            .unwrap_or_else(|| Map::new(&env));

        history.set(tx_ref.clone(), (from.clone(), to.clone(), amount));
        env.storage()
            .persistent()
            .set(&StorageKey::TransactionHistory, &history);

        // Emit event for off-chain listeners / UI updates
        env.events().publish(
            (symbol_short!("PAYMENT"), tx_ref),
            (from, to, amount),
        );

        log!(&env, "FARE_PAID: ref={}, amount={}", tx_ref, amount);
    }

    /// Verify if a specific transaction reference exists and was paid.
    pub fn verify_payment(env: Env, tx_ref: Symbol) -> bool {
        let history: Map<Symbol, (Address, Address, i128)> = env
            .storage()
            .persistent()
            .get(&StorageKey::TransactionHistory)
            .unwrap_or_else(|| Map::new(&env));

        let exists = history.contains_key(tx_ref);
        log!(&env, "PAYMENT_VERIFIED: result={}", exists);
        exists
    }

    /// Get transaction details
    pub fn get_transaction(env: Env, tx_ref: Symbol) -> (Address, Address, i128) {
        let history: Map<Symbol, (Address, Address, i128)> = env
            .storage()
            .persistent()
            .get(&StorageKey::TransactionHistory)
            .unwrap_or_else(|| Map::new(&env));

        history.get(tx_ref).unwrap_or_else(|| panic!("Tx not found"))
    }
}
