// Import necessary types from the Soroban SDK
#![allow(unused)]
use soroban_sdk::{contracttype, contracterror, xdr::ToXdr, Address, Bytes, BytesN, Env};

soroban_sdk::contractimport!(
    file = "../pair/target/wasm32-unknown-unknown/release/raumfi_pair.wasm"
);

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum IdenticalPairError {
    /// RaumFiFactory: token_a and token_b have identical addresses
    CreatePairIdenticalTokens = 901,
}

#[contracttype]
#[derive(Clone)]
pub struct Pair(Address, Address);

impl Pair {
    pub fn new(a: Address, b: Address) -> Result<Self, IdenticalPairError> {
        if a == b {
            return Err(IdenticalPairError::CreatePairIdenticalTokens);
        }
        if a < b {
            Ok(Pair(a, b))
        } else {
            Ok(Pair(b, a))
        }
    }

    pub fn salt(&self, e: &Env) -> BytesN<32> {
        let mut salt = Bytes::new(e);
        salt.append(&self.0.clone().to_xdr(e));
        salt.append(&self.1.clone().to_xdr(e));
        e.crypto().sha256(&salt)
    }

    pub fn token_0(&self) -> &Address {
        &self.0
    }

    pub fn token_1(&self) -> &Address {
        &self.1
    }
}

// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env,                    // Pass in the current environment as an argument
    pair_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
    token_pair: &Pair,
) -> Address {
    e.deployer()
        .with_current_contract(token_pair.salt(&e))
        .deploy(pair_wasm_hash)
}
