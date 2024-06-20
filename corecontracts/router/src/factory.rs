soroban_sdk::contractimport!(
    file = "../factory/target/wasm32-unknown-unknown/release/raumfi_factory.wasm"
);
pub type RaumFiFactoryClient<'a> = Client<'a>;