use soroban_sdk::{Address, Env};
use crate::pair_token::storage_types::{DataKey};
const DAY_IN_LEDGERS: u32 = 17280;
const BALANCE_BUMP_AMOUNT: u32 = 120 * DAY_IN_LEDGERS;
const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn read_balance(e: &Env, address: Address) -> i128 {
    let key = DataKey::Balance(address);
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        balance
    } else {
        0
    }
}

fn write_balance(e: &Env, address: Address, amount: i128) {
    let key = DataKey::Balance(address);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn receive_balance(e: &Env, address: Address, amount: i128) {
    let balance = read_balance(e, address.clone());

    let new_balance = balance.checked_add(amount)
        .expect("Integer overflow occurred");

    write_balance(e, address, new_balance);
}

pub fn spend_balance(e: &Env, address: Address, amount: i128) {
    let balance = read_balance(e, address.clone());
    if balance < amount {
        panic!("Balance is less than the spending amount");
    }
    write_balance(e, address, balance - amount);
}
