soroban_sdk::contractimport!(
    file = "../pair/target/wasm32-unknown-unknown/release/raumfi_pair.wasm"
);
pub type RaumFiPairClient<'a> = Client<'a>;