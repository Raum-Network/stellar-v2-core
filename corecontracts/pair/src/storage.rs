use soroban_sdk::{ Env, Address, ConversionError,
    TryFromVal, Val}; 

#[derive(Clone, Copy)] 
#[repr(u32)]

pub enum DataKey {
    Token0 = 0, // token0, instance type of data;
    Token1 = 1, // token1, instance type of data;
    Reserve0 = 2, // reserve0, instance type of data;
    Reserve1 = 3, // reserve1, instance type of data;
    Factory = 4, // factory, instance type of data;
    Multiplier = 5 // last value of multiplier i.e k, instance type of data;

}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn get_factory(e: &Env) -> Address {
    e.storage().instance().
    get(&DataKey::Factory).unwrap()
}

pub fn has_token_0(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Token0)
}

pub fn get_token_0(e: &Env) -> Address {
    e.storage().instance().
get(&DataKey::Token0).unwrap()
}

pub fn get_token_1(e: &Env) -> Address {
    e.storage().instance().
get(&DataKey::Token1).unwrap()
}

pub fn get_reserve_0(e: &Env) -> i128 {
    e.storage().instance().
get(&DataKey::Reserve0).unwrap()
}

pub fn get_reserve_1(e: &Env) -> i128 {
    e.storage().instance().
get(&DataKey::Reserve1).unwrap()
}



pub fn get_multiplier(e: &Env) -> i128 {
    if let Some(multiplier) = e.storage().instance().
get(&DataKey::Multiplier) {
        multiplier
    } else {
        0
    }
}

pub fn put_factory(e: &Env, factory: Address) {
    e.storage().instance().
set(&DataKey::Factory, &factory);
}

pub fn put_token_0(e: &Env, contract_id: Address) {
    e.storage().instance().
set(&DataKey::Token0, &contract_id);
}

pub fn put_token_1(e: &Env, contract_id: Address) {
    e.storage().instance().
set(&DataKey::Token1, &contract_id);
}

pub fn put_reserve_0(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("Amount Cannot Be Negative")
    }
    e.storage().instance().
set(&DataKey::Reserve0, &amount)
}

pub fn put_reserve_1(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("Amount Cannot Be Negative")
    }
    e.storage().instance().
set(&DataKey::Reserve1, &amount)
}

pub fn put_multiplier(e: &Env, multiplier: i128) {
    e.storage().instance().
set(&DataKey::Multiplier, &multiplier);
}