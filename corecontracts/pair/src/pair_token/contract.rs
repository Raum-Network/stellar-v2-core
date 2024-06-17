//! This contract demonstrates a sample implementation of the Soroban token
//! interface using Soroban SDK.
//! Inspired By Soroban Token Example
use crate::pair_token::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::pair_token::balance::{read_balance, receive_balance, spend_balance};
use crate::pair_token::metadata::{read_decimal, read_name, read_symbol};
use crate::pair_token::total_supply::{read_total_supply, increase_total_supply, decrease_total_supply};

#[cfg(test)]
use crate::pair_token::storage_types::{AllowanceDataKey, AllowanceValue, DataKey};
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use soroban_token_sdk::TokenUtils;

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

fn is_amount_negative(amount: i128) {
    if amount < 0 {
        panic!("amount cannot be less than 0 -> : {}", amount)
    }
}

pub fn internal_burn(e: Env, address: Address, amount: i128) {
    is_amount_negative(amount);
 
    e.storage()
    .instance()
    .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    
    spend_balance(&e, address.clone(), amount);
    decrease_total_supply(&e, amount);

    TokenUtils::new(&e).events().burn(address, amount);
} 

pub fn internal_mint(e: Env, to: Address, amount: i128) {
    is_amount_negative(amount);

    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        
    receive_balance(&e, to.clone(), amount);
    increase_total_supply(&e, amount);

    TokenUtils::new(&e).events().mint(e.current_contract_address(), to, amount);
}


#[contract]
pub struct PairToken;

#[contractimpl]
impl PairToken {

    pub fn total_supply(e: Env) -> i128 {
        read_total_supply(&e)
    }

    #[cfg(test)]
    pub fn get_allowance(e: Env, from: Address, spender: Address) -> Option<AllowanceValue> {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        let allowance = e.storage().temporary().get::<_, AllowanceValue>(&key);
        allowance
    }
}

#[contractimpl]
impl token::Interface for PairToken { 
    fn allowance(e: Env, address: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, address, spender).amount
    }

    fn approve(e: Env, address: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        address.require_auth();

        is_amount_negative(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, address.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(address, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, address: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, address)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        is_amount_negative(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        is_amount_negative(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, address: Address, amount: i128) {
        address.require_auth();
        internal_burn(e, address, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        is_amount_negative(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        decrease_total_supply(&e, amount);

        TokenUtils::new(&e).events().burn(from, amount)
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
