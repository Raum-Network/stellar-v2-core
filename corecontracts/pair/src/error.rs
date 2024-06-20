use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]

pub enum PairError {
    /// Pair: already initialized
    InitializeAlreadyInitialized = 101,
    /// Pair: not yet initialized
    NotInitialized = 102,

    /// Pair: token_0 must be less than token_1 while initializing
    InitializeTokenOrderInvalid = 103,

    /// Pair: insufficient amount of token 0 sent while doing deposit
    DepositInsufficientAmountToken0 = 104,
    /// Pair: insufficient amount of token 1 sent while doing deposit
    DepositInsufficientAmountToken1 = 105,
    /// Pair: insufficient first liquidity minted while doing deposit
    DepositInsufficientFirstLiquidity = 106,
    /// Pair: insufficient liquidity minted while doing deposit
    DepositInsufficientLiquidityMinted = 107,
    /// Pair: insufficient output amount while doing deposDepositit

    SwapInsufficientOutputAmount = 108,
    /// Pair: negatives amounts out dont supported while doing swap
    SwapNegativesOutNotSupported = 109,
    /// Pair: insufficient liquidity to do the swap
    SwapInsufficientLiquidity = 110,
    /// Pair: invalid to to do the swap
    SwapInvalidTo = 111,
    /// Pair: insufficient input amount while doing swap
    SwapInsufficientInputAmount = 112,
    /// Pair: negatives amounts in dont supported while doing swap
    SwapNegativesInNotSupported = 113,
    /// Pair: Multiplier is not met while doing swap
    SwapConstantNotMet = 114,

    /// Pair: liquidity was not initialized yet while doing withdraw
    WithdrawLiquidityNotInitialized = 115,
    /// Pair: insufficient sent shares while doing withdraw
    WithdrawInsufficientSentShares = 116,
    /// Pair: insufficient liquidity burned while doing withdraw
    WithdrawInsufficientLiquidityBurned = 117,

    /// Pair: OVERFLOW while updating
    UpdateOverflow = 118,
}