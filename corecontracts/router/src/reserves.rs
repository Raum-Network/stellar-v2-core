use soroban_sdk::{Address, Env};
use crate::tokens::{sort_tokens, pair_for};
use crate::routererrors::RaumFiLibraryError;

mod pair {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/raumfi_pair.wasm"
    );
}
use pair::Client as RaumFiPairClient;


/// Fetches and sorts the reserves for a pair of tokens.
///
/// # Returns
///
/// Returns `Result<(i128, i128), RaumFiLibraryError>` where `Ok` contains a tuple of sorted reserves, and `Err` indicates an error such as identical tokens or an issue with sorting.
pub fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> Result<(i128,i128), RaumFiLibraryError>{
    let (token_0,token_1) = sort_tokens(token_a.clone(), token_b.clone())?;
    let pair_address = pair_for(e.clone(), factory, token_0.clone(), token_1.clone())?;
    let pair_client = RaumFiPairClient::new(&e, &pair_address);
    let (reserve_0, reserve_1) = pair_client.get_reserves();
    
    let (reserve_a, reserve_b) =
        if token_a == token_0 {
            (reserve_0, reserve_1) 
        } else {
            (reserve_1, reserve_0) };

    Ok((reserve_a, reserve_b))
}