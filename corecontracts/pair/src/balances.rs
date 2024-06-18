use soroban_sdk::{Address, Env}; 
use crate::{pair_token::{PairToken}, any_token};
use crate::storage::*;
use soroban_sdk::token::Interface;


pub fn get_balance(e: &Env, contract_id: Address) -> i128 {

    any_token::TokenClient::new(e, &contract_id).balance(&e.current_contract_address())
}

pub fn get_balance_0(e: &Env) -> i128 {
    get_balance(e, get_token_0(e))
}

pub fn get_balance_1(e: &Env) -> i128 {
    get_balance(e, get_token_1(e))
}

pub fn get_lp_token_balance(e: &Env) -> i128 {
    PairToken::balance(e.clone(), e.current_contract_address())
}