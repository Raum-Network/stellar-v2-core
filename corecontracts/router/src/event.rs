//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, Vec};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub factory: Address
}

pub(crate) fn initialized(e: &Env, factory: Address) {
    
    let event: InitializedEvent = InitializedEvent {
        factory: factory
    };
    e.events().publish(("RaumFiRouter", symbol_short!("init")), event);
}

// ADD LIQUIDITY EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddLiquidityEvent {
    pub token_a: Address,
    pub token_b: Address,
    pub pair: Address,
    pub amount_a: i128,
    pub amount_b: i128,
    pub liquidity: i128,
    pub to: Address
}

/// Publishes an `AddLiquidityEvent` to the event stream.

pub(crate) fn add_liquidity(
    e: &Env,
    token_a: Address,
    token_b: Address,
    pair: Address,
    amount_a: i128,
    amount_b: i128,
    liquidity: i128,
    to: Address,
) {
    let event = AddLiquidityEvent {
        token_a,
        token_b,
        pair,
        amount_a,
        amount_b,
        liquidity,
        to,
    };

    e.events().publish(("RaumFiRouter", symbol_short!("add")), event);
}

 

// REMOVE LIQUIDITY EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveLiquidityEvent {
    pub token_a: Address,
    pub token_b: Address,
    pub pair: Address,
    pub amount_a: i128,
    pub amount_b: i128,
    pub liquidity: i128,
    pub to: Address
}


/// Publishes an `RemoveLiquidityEvent` to the event stream.

pub(crate) fn remove_liquidity(
    e: &Env,
    token_a: Address,
    token_b: Address,
    pair: Address,
    amount_a: i128,
    amount_b: i128,
    liquidity: i128,
    to: Address,
) {
    let event = RemoveLiquidityEvent {
        token_a,
        token_b,
        pair,
        amount_a,
        amount_b,
        liquidity,
        to,
    };

    e.events().publish(("RaumFiRouter", symbol_short!("remove")), event);
}



// SWAP EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub path: Vec<Address>,
    pub amounts: Vec<i128>,
    pub to: Address
}

/// Publishes an `SwapEvent` to the event stream.

pub(crate) fn swap(
    e: &Env,
    path: Vec<Address>,
    amounts: Vec<i128>,
    to: Address
) {
    let event = SwapEvent {
        path,
        amounts,
        to,
    };

    e.events().publish(("RaumFiRouter", symbol_short!("swap")), event);
}