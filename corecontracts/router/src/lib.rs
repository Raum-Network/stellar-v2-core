#![no_std]
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};
use crate::routererrors::{RaumFiLibraryError};
use crate::tokens::{sort_tokens, pair_for};

mod pair;
mod factory;
mod event;
mod storage;
mod error;
mod router_errors;
mod reserves;
mod quotes;
mod tokens;
mod math;

use factory::RaumFiFactoryClient;
use pair::RaumFiPairClient;
use storage::{put_factory, has_factory, get_factory, extend_instance_ttl};
pub use error::{RaumFiRouterError, RouterErrorsForLibrary};

pub fn is_amount_negative(amount: i128) -> Result<(), RouterErrorsForLibrary> {
    if amount < 0 {
        Err(RouterErrorsForLibrary::RouterNegativeNotAllowed)
    } else {
        Ok(())
    }
}


fn is_deadline_expired(e: &Env, timestamp: u64) -> Result<(), RouterErrorsForLibrary> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(RaumFiRouterError::DeadlineExpired.into())
    } else {
        Ok(())
    }
}


fn is_initialized(e: &Env) -> Result<(), RouterErrorsForLibrary> {
    if has_factory(e) {
        Ok(())
    } else {
        Err(RouterErrorsForLibrary::RouterNotInitialized)
    }
}


/// Given a pair of tokens, a desired and minimum amount of tokens to provide as liquidity, this function calculates
/// the correct amounts of tokens to add to the pool. If the pool doesn't exist, it creates one.
///
/// It considers the desired and minimum amounts for both tokens and calculates the optimal distribution to
/// satisfy these requirements while taking into account the current reserves in the pool.
///
///
/// # Returns
/// A tuple containing the calculated amounts of token A and B to be added to the pool.
fn add_liquidity_amounts(
    e: Env,
    factory: Address,
    token_a: Address,
    token_b: Address,
    amount_a_desired: i128,
    amount_b_desired: i128,
    amount_a_min: i128,
    amount_b_min: i128,
) -> Result<(i128, i128), RouterErrorsForLibrary> {
    // checks if the pair exists; otherwise, creates the pair
    let factory_client = RaumFiFactoryClient::new(&e, &factory);
    if !factory_client.pair_exists(&token_a, &token_b) {
        factory_client.create_pair(&token_a, &token_b);
    }

    let (token_0,token_1) = sort_tokens(token_a.clone(), token_b.clone())?;
    let pair_address = pair_for(e.clone(), factory.clone(), token_0.clone(), token_1.clone())?;
    let pair_client = RaumFiPairClient::new(&e, &pair_address);
    let (reserve_0, reserve_1) = pair_client.get_reserves();
    
    let (reserve_a, reserve_b) =
        if token_a == token_0 {
            (reserve_0, reserve_1) 
        } else {
            (reserve_1, reserve_0) };

    // When there is no liquidity (first deposit)
    if reserve_a == 0 && reserve_b == 0 {
        Ok((amount_a_desired, amount_b_desired))
    } else {
        
        let amount_b_optimal = quotes::quote(
            amount_a_desired.clone(),
            reserve_a.clone(),
            reserve_b.clone(),
        )?;

        if amount_b_optimal <= amount_b_desired {
            if amount_b_optimal < amount_b_min {
                return Err(RaumFiRouterError::InsufficientBAmount.into());
            }
            Ok((amount_a_desired, amount_b_optimal))
        }
        // If not, we can try with the amount b desired
        else {
            let amount_a_optimal = quotes::quote(amount_b_desired, reserve_b, reserve_a).map_err(RaumFiLibraryError::from)?;

            assert!(amount_a_optimal <= amount_a_desired);

            if amount_a_optimal < amount_a_min {
                return Err(RaumFiRouterError::InsufficientAAmount.into());
            }
            Ok((amount_a_optimal, amount_b_desired))
        }
    }
}

/// Executes a series of token swaps along the provided trading route.
/// Requires that the initial amount has already been sent to the first pair in the route.
///

fn swap(e: &Env, factory_address: &Address, amounts: &Vec<i128>, path: &Vec<Address>, _to: &Address) -> Result<(), RouterErrorsForLibrary>{
    for i in 0..path.len() - 1 {
        //  represents a half-open range, which includes the start value (0) but excludes the end value (path.len() - 1)
        let (input, output): (Address, Address) = (path.get(i).unwrap(), path.get(i + 1).unwrap());

        let (token_0, _token_1): (Address, Address) =
            (sort_tokens(input.clone(), output.clone()))?;
        
            let amount_out: i128 = amounts.get(i + 1).unwrap();

        let (amount_0_out, amount_1_out): (i128, i128) = if input == token_0 {
            (0, amount_out)
        } else {
            (amount_out, 0)
        };

        // before the end, "to" must be the next pair... "to" will be the user only at the end
        let to: Address = if i < path.len() - 2 {
            pair_for(
                e.clone(),
                factory_address.clone(),
                output.clone(),
                path.get(i + 2).unwrap(),
            )?
        } else {
            _to.clone()
        };

        RaumFiPairClient::new(
            &e,
            &pair_for(e.clone(), factory_address.clone(), input, output)?,
        )
        .swap(&amount_0_out, &amount_1_out, &to);

    }

    Ok(())
}


/*
    RaumFi ROUTER SMART CONTRACT INTERFACE:
*/

pub trait RaumFiRouterTrait {

    /// Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address) -> Result<(), RouterErrorsForLibrary>;

    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> Result<(i128, i128, i128), RouterErrorsForLibrary>;

    fn remove_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        liquidity: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> Result<(i128, i128), RouterErrorsForLibrary>;


    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, RouterErrorsForLibrary>;

    
    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, RouterErrorsForLibrary>;

    fn get_factory(e: Env) -> Result<Address, RouterErrorsForLibrary>;

    fn router_pair_for(e: Env, token_a: Address, token_b: Address) -> Result<Address, RouterErrorsForLibrary>;

    fn router_quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> Result<i128, RouterErrorsForLibrary>;

    fn router_get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> Result<i128, RouterErrorsForLibrary>;

    fn router_get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> Result<i128, RouterErrorsForLibrary>;

    fn router_get_amounts_out(e: Env, amount_in: i128, path: Vec<Address>) -> Result<Vec<i128>, RouterErrorsForLibrary>;
    
    fn router_get_amounts_in(e: Env, amount_out: i128, path: Vec<Address>) -> Result<Vec<i128>, RouterErrorsForLibrary>;

    

}

#[contract]
struct RaumFiRouter;

#[contractimpl]
impl RaumFiRouterTrait for RaumFiRouter {
    /// Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address) -> Result<(), RouterErrorsForLibrary> {
        if !has_factory(&e) {
            put_factory(&e, &factory);
            event::initialized(&e, factory);
            extend_instance_ttl(&e);
            Ok(())
        } else {
            Err(RaumFiRouterError::InitializeAlreadyInitialized.into())
        } 
        
    }  

    /// Adds liquidity to a token pair's pool, creating it if it doesn't exist. Ensures that exactly the desired amounts
    /// of both tokens are added, subject to minimum requirements.
    /// This function is responsible for transferring tokens from the user to the pool and minting liquidity tokens in return.
    /// # Returns
    /// A tuple containing: amounts of token A and B added to the pool.
    /// plus the amount of liquidity tokens minted.
    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> Result<(i128, i128, i128), RouterErrorsForLibrary> {
        is_initialized(&e)?;
        is_amount_negative(amount_a_desired)?;
        is_amount_negative(amount_b_desired)?;
        is_amount_negative(amount_a_min)?;
        is_amount_negative(amount_b_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        is_deadline_expired(&e, deadline)?;

        let factory = get_factory(&e);

        let (amount_a, amount_b) = add_liquidity_amounts(
            e.clone(),
            factory.clone(),
            token_a.clone(),
            token_b.clone(),
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
        )?;

        let pair: Address = pair_for(
            e.clone(),
            factory,
            token_a.clone(),
            token_b.clone(),
        ).map_err(RaumFiLibraryError::from)?;

        TokenClient::new(&e, &token_a).transfer(&to, &pair, &amount_a);
        TokenClient::new(&e, &token_b).transfer(&to, &pair, &amount_b);

        let liquidity = RaumFiPairClient::new(&e, &pair).deposit(&to);

        event::add_liquidity(
            &e,
            token_a,
            token_b,
            pair,
            amount_a,
            amount_b,
            liquidity,
            to);
            
        Ok((amount_a, amount_b, liquidity))
    }

    /// Removes liquidity from a token pair's pool.
    ///
    /// This function facilitates the removal of liquidity from a RaumFi Liquidity Pool by burning a specified amount
    /// of Liquidity Pool tokens (`liquidity`) owned by the caller. In return, it transfers back the corresponding
    /// amounts of the paired tokens (`token_a` and `token_b`) to the caller's specified address (`to`).
    ///
    /// # Returns
    /// A tuple containing the amounts of `token_a` and `token_b` withdrawn from the pool.
    fn remove_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        liquidity: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> Result<(i128, i128), RouterErrorsForLibrary> {
        is_initialized(&e)?;
        is_amount_negative(liquidity)?;
        is_amount_negative(amount_a_min)?;
        is_amount_negative(amount_b_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        is_deadline_expired(&e, deadline)?;

        // Ensure that the pair exists in the RaumFi factory
        let factory_address = get_factory(&e);
        let factory = RaumFiFactoryClient::new(&e, &factory_address);

        if !factory.pair_exists(&token_a, &token_b) {
            return Err(RaumFiRouterError::PairDoesNotExist.into());
        }

        // Retrieve the pair's contract address using the RaumFi library
        let pair: Address = pair_for(
            e.clone(),
            get_factory(&e),
            token_a.clone(),
            token_b.clone(),
        )?;

        // Transfer LP tokens from the caller to the pair contract
        TokenClient::new(&e, &pair).transfer(&to, &pair, &liquidity);
        
        // Withdraw paired tokens from the pool
        let (amount_0, amount_1) = RaumFiPairClient::new(&e, &pair).withdraw(&to);

        // Sort tokens to match the expected order
        let (token_0, _token_1) = sort_tokens(token_a.clone(), token_b.clone())?;
        let (amount_a, amount_b) = if token_a == token_0 {
            (amount_0, amount_1)
        } else {
            (amount_1, amount_0)
        };

        // Check if the received amounts meet the minimum requirements
        if amount_a < amount_a_min {
            return Err(RaumFiRouterError::InsufficientAAmount.into());
        }
        if amount_b < amount_b_min {
            return Err(RaumFiRouterError::InsufficientBAmount.into());
        }

        event::remove_liquidity(
            &e,
            token_a,
            token_b,
            pair,
            amount_a,
            amount_b,
            liquidity,
            to);

        // Return the amounts of paired tokens withdrawn
        Ok((amount_a, amount_b))
    }

    /// Swaps an exact amount of input tokens for as many output tokens as possible
    /// along the specified trading route. The route is determined by the `path` vector,
    /// where the first element is the input token, the last is the output token, 
    /// and any intermediate elements represent pairs to trade through if a direct pair does not exist.
    ///
    /// # Returns
    /// A vector containing the amounts of tokens received at each step of the trading route.
    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, RouterErrorsForLibrary> {
        is_initialized(&e)?;
        is_amount_negative(amount_in)?;
        is_amount_negative(amount_out_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        is_deadline_expired(&e, deadline)?;

        // Get the expected output amounts for each step of the trading route        
        let factory_address = get_factory(&e);
        let amounts = quotes::get_amounts_out(
            e.clone(),
            factory_address.clone(),
            amount_in,
            path.clone(),
        )?;

        // Ensure that the final output amount meets the minimum requirement        
        if amounts.get(amounts.len() - 1).unwrap() < amount_out_min {
            return Err(RaumFiRouterError::InsufficientOutputAmount.into());
        }
        
        // Determine the pair contract address for the first step of the trading route
        let pair = pair_for(
            e.clone(),
            factory_address.clone(),
            path.get(0).unwrap(),
            path.get(1).unwrap(),
        )?;
        
        // Transfer input tokens to the pair contract
        // If the pair does not exist, this will fail here: Should be implement factory.pair_exists?
        // If we implement, we will include an additional cross-contract call...
        TokenClient::new(&e, &path.get(0).unwrap()).transfer(&to, &pair, &amounts.get(0).unwrap());

        // Execute the tokens swap
        swap(&e, &factory_address, &amounts, &path, &to)?;
    
        event::swap(
            &e,
            path,
            amounts.clone(),
            to);

        // Return the amounts of tokens received at each step of the trading route
        Ok(amounts)
    }

    /// Swaps tokens for an exact amount of output token, following the specified trading route.
    /// The route is determined by the `path` vector, where the first element is the input token,
    /// the last is the output token, and any intermediate elements represent pairs to trade through.
    /// 
    /// # Returns
    /// A vector containing the amounts of tokens used at each step of the trading route.
    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, RouterErrorsForLibrary> {
        is_initialized(&e)?;
        is_amount_negative(amount_out)?;
        is_amount_negative(amount_in_max)?;
        extend_instance_ttl(&e);
        to.require_auth(); 
        is_deadline_expired(&e, deadline)?;

        // Get the expected input amounts for each step of the trading route
        let factory_address = get_factory(&e);
        let amounts = quotes::get_amounts_in(
            e.clone(),
            factory_address.clone(),
            amount_out,
            path.clone(),
        )?;
        
        // Ensure that the input amount does not exceed the maximum allowed
        if amounts.get(0).unwrap() > amount_in_max {
            return Err(RaumFiRouterError::ExcessiveInputAmount.into());
        }

        // Determine the pair contract address for the first step of the trading route
        let pair = pair_for(
            e.clone(),
            factory_address.clone(),
            path.get(0).unwrap(),
            path.get(1).unwrap(),
        )?;
        // Transfer input tokens to the pair contract
        // If the pair does not exist, this will fail here: Should be implement factory.pair_exists?
        // If we implement, we will include an additional cross-contract call...
        TokenClient::new(&e, &path.get(0).unwrap()).transfer(&to, &pair, &amounts.get(0).unwrap());

        // Execute the token swap
        swap(&e, &factory_address, &amounts, &path, &to)?;
    
        event::swap(
            &e,
            path,
            amounts.clone(),
            to);

        // Return the amounts of tokens used at each step of the trading route
        Ok(amounts)
    }

    /*  *** Read only functions: *** */


    /// This function retrieves the factory contract's address associated with the provided environment.
    /// It also checks if the factory has been initialized and raises an assertion error if not.
    /// If the factory is not initialized, this code will raise an assertion error with the message "RaumFiRouter: not yet initialized".
    /// # Arguments
    /// * `e` - The contract environment (`Env`) in which the contract is executing.
    fn get_factory(e: Env) -> Result<Address, RouterErrorsForLibrary> {
        is_initialized(&e)?; 
        extend_instance_ttl(&e);
        let factory_address = get_factory(&e);
        Ok(factory_address)
    }


    /// Calculates the deterministic address for a pair without making any external calls.
    /// check <https://github.com/paltalabs/deterministic-address-soroban>
    ///
    /// # Returns
    ///
    /// Returns `Result<Address, RaumFiLibraryError>` where `Ok` contains the deterministic address for the pair, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn router_pair_for(e: Env, token_a: Address, token_b: Address) -> Result<Address, RouterErrorsForLibrary> {
        extend_instance_ttl(&e);
        Ok(pair_for(
            e.clone(),
            get_factory(&e),
            token_a.clone(),
            token_b.clone(),
        )?)
    }


    /// Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, RaumFiLibraryError>` where `Ok` contains the calculated equivalent amount, and `Err` indicates an error such as insufficient amount or liquidity
    fn router_quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> Result<i128, RouterErrorsForLibrary> {
        Ok(quotes::quote(amount_a, reserve_a, reserve_b)?)
    }

    /// Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, RaumFiLibraryError>` where `Ok` contains the calculated maximum output amount, and `Err` indicates an error such as insufficient input amount or liquidity.
    fn router_get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> Result<i128, RouterErrorsForLibrary> {
        Ok(quotes::get_amount_out(amount_in, reserve_in, reserve_out)?)
    }

    /// Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, RaumFiLibraryError>` where `Ok` contains the required input amount, and `Err` indicates an error such as insufficient output amount or liquidity.
    fn router_get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> Result<i128, RouterErrorsForLibrary> {
        Ok(quotes::get_amount_in(amount_out, reserve_in, reserve_out)?)
    }


    /// Performs chained get_amount_out calculations on any number of pairs.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, RaumFiLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn router_get_amounts_out(e: Env, amount_in: i128, path: Vec<Address>) -> Result<Vec<i128>, RouterErrorsForLibrary> {
        is_initialized(&e)?;
        extend_instance_ttl(&e);
        let factory = get_factory(&e);
        Ok(quotes::get_amounts_out(e, factory, amount_in, path)?)
    }

    /// Performs chained get_amount_in calculations on any number of pairs.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, RaumFiLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn router_get_amounts_in(e: Env, amount_out: i128, path: Vec<Address>) -> Result<Vec<i128>, RouterErrorsForLibrary> {
        is_initialized(&e)?;
        extend_instance_ttl(&e);
        let factory = get_factory(&e);
        Ok(quotes::get_amounts_in(e, factory, amount_out, path)?)
    }


}
