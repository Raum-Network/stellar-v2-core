use soroban_sdk::{contracttype, Address, BytesN, Env, Val, TryFromVal};
use crate::factory_error::FactoryError;
use crate::pair::Pair;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    FeeTo,
    FeeToSetter,
    PairWasmHash,
    FeesEnabled,
    TotalPairs,
    PairAddressesNIndexed(u32),
    PairAddressesByTokens(Pair),
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

const PERSISTENT_BUMP_AMOUNT: u32 = 60 * DAY_IN_LEDGERS;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

fn get_persistent_extend_or_error<V: TryFromVal<Env, Val>>(
    e: &Env,
    key: &DataKey,
    error: FactoryError,
) -> Result<V, FactoryError> {
    if let Some(result) = e.storage().persistent().get(key) {
        e.storage()
            .persistent()
            .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        result
    } else {
        Err(error)
    }
}

// TotalPairs
pub fn put_total_pairs(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalPairs, &n);
}

pub fn get_total_pairs(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalPairs).unwrap_or(0)
}

pub fn has_total_pairs(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::TotalPairs)
}

// PairAddressesByTokens
pub fn put_pair_address_by_token_pair(e: &Env, token_pair: Pair, pair_address: &Address) {
    let key = DataKey::PairAddressesByTokens(token_pair);
    e.storage().persistent().set(&key, pair_address);
    e.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn get_pair_address_by_token_pair(e: &Env, token_pair: Pair) -> Result<Address, FactoryError> {
    let key = DataKey::PairAddressesByTokens(token_pair);
    get_persistent_extend_or_error(e, &key, FactoryError::PairDoesNotExist)
}

pub fn get_pair_exists(e: &Env, token_pair: Pair) -> bool {
    let key = DataKey::PairAddressesByTokens(token_pair);
    if e.storage().persistent().has(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        true
    } else {
        false
    }
}

// FeeTo
pub fn get_fee_to(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::FeeTo).unwrap()
}

// FeesEnabled
pub fn get_fees_enabled(e: &Env) -> bool {
    e.storage().instance().get(&DataKey::FeesEnabled).unwrap_or(false)
}

// FeeToSetter
pub fn get_fee_to_setter(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::FeeToSetter).unwrap()
}

// PairWasmHash
pub fn get_pair_wasm_hash(e: &Env) -> Result<BytesN<32>, FactoryError> {
    let key = DataKey::PairWasmHash;
    get_persistent_extend_or_error(e, &key, FactoryError::NotInitialized)
}

// Setters
pub fn put_fee_to(e: &Env, to: Address) {
    e.storage().instance().set(&DataKey::FeeTo, &to);
}

pub fn put_fee_to_setter(e: &Env, setter: &Address) {
    e.storage().instance().set(&DataKey::FeeToSetter, setter);
}

pub fn put_fees_enabled(e: &Env, is_enabled: &bool) {
    e.storage().instance().set(&DataKey::FeesEnabled, is_enabled);
}

pub fn put_pair_wasm_hash(e: &Env, pair_wasm_hash: BytesN<32>) {
    let key = DataKey::PairWasmHash;
    e.storage().persistent().set(&key, &pair_wasm_hash);
    e.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn add_pair_to_all_pairs(e: &Env, pair_address: &Address) {
    let mut total_pairs = get_total_pairs(e);
    let key = DataKey::PairAddressesNIndexed(total_pairs);
    e.storage().persistent().set(&key, pair_address);
    e.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    total_pairs = total_pairs.checked_add(1).expect("Integer Overflow");
    put_total_pairs(e, total_pairs);
}

pub fn get_all_pairs(e: Env, n: u32) -> Result<Address, FactoryError> {
    let key = DataKey::PairAddressesNIndexed(n);
    get_persistent_extend_or_error(&e, &key, FactoryError::IndexDoesNotExist)
}

