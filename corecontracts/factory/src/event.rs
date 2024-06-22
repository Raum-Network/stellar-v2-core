//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, Symbol, Env, Address};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub setter: Address
}

pub(crate) fn initialized(e: &Env, setter: Address) {
    
    let event: InitializedEvent = InitializedEvent {
        setter: setter
    };
    e.events().publish(("RaumFiFactory", Symbol::new(e , "initialize_factory_contract")), event);
}

// NEW PAIR CREATED EVENT: new_pair
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PairEvent {
    pub pair: Address,
    pub token_0: Address,
    pub token_1: Address,
    pub new_pairs_length: u32
}

pub(crate) fn new_pair(
    e: &Env, 
    token_0: Address,
    token_1: Address,
    pair: Address,
    new_pairs_length: u32) {
    
    let event: PairEvent = PairEvent {
        pair: pair,
        token_0: token_0,
        token_1: token_1,
        new_pairs_length: new_pairs_length,
    };
    e.events().publish(("RaumFiFactory", Symbol::new(e , "new_pair_created")), event);
}
