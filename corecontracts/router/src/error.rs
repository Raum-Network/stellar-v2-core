use soroban_sdk::{self, contracterror};
use crate::router_errors::RaumFiLibraryError;


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RaumFiRouterError {
    /// RaumFiRouter: not yet initialized
    NotInitialized = 401,

    /// RaumFiRouter: negative amount is not allowed
    NegativeNotAllowed = 402,

    /// RaumFiRouter: deadline expired
    DeadlineExpired = 403,
    
    /// RaumFiRouter: already initialized
    InitializeAlreadyInitialized = 404,

    /// RaumFiRouter: insufficient a amount
    InsufficientAAmount = 405,

    /// RaumFiRouter: insufficient b amount
    InsufficientBAmount = 406,

    /// RaumFiRouter: insufficient output amount
    InsufficientOutputAmount = 407,

    /// RaumFiRouter: excessive input amount
    ExcessiveInputAmount = 408,

    /// RaumFiRouter: pair does not exist
    PairDoesNotExist = 409,

}


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
// Define a new set of integer literals for the CombinedError enum
pub enum RouterErrorsForLibrary {
    RouterNotInitialized = 501,
    RouterNegativeNotAllowed = 502,
    RouterDeadlineExpired = 503,
    RouterInitializeAlreadyInitialized = 504,
    RouterInsufficientAAmount = 505,
    RouterInsufficientBAmount = 506,
    RouterInsufficientOutputAmount = 507,
    RouterExcessiveInputAmount = 508,
    RouterPairDoesNotExist = 509,

    LibraryInsufficientAmount = 510,
    LibraryInsufficientLiquidity = 511,
    LibraryInsufficientInputAmount = 512,
    LibraryInsufficientOutputAmount = 513,
    LibraryInvalidPath = 514,
    LibrarySortIdenticalTokens = 515,
}

impl From<RaumFiLibraryError> for RouterErrorsForLibrary {
    fn from(err: RaumFiLibraryError) -> Self {
        match err {
            RaumFiLibraryError::InsufficientAmount => RouterErrorsForLibrary::LibraryInsufficientAmount,
            RaumFiLibraryError::InsufficientLiquidity => RouterErrorsForLibrary::LibraryInsufficientLiquidity,
            RaumFiLibraryError::InsufficientInputAmount => RouterErrorsForLibrary::LibraryInsufficientInputAmount,
            RaumFiLibraryError::InsufficientOutputAmount => RouterErrorsForLibrary::LibraryInsufficientOutputAmount,
            RaumFiLibraryError::InvalidPath => RouterErrorsForLibrary::LibraryInvalidPath,
            RaumFiLibraryError::SortIdenticalTokens => RouterErrorsForLibrary::LibrarySortIdenticalTokens,
        }
    }
}

impl From<RaumFiRouterError> for RouterErrorsForLibrary {
    fn from(err: RaumFiRouterError) -> Self {
        match err {
            RaumFiRouterError::NotInitialized => RouterErrorsForLibrary::RouterNotInitialized,
            RaumFiRouterError::NegativeNotAllowed => RouterErrorsForLibrary::RouterNegativeNotAllowed,
            RaumFiRouterError::DeadlineExpired => RouterErrorsForLibrary::RouterDeadlineExpired,
            RaumFiRouterError::InitializeAlreadyInitialized => RouterErrorsForLibrary::RouterInitializeAlreadyInitialized,
            RaumFiRouterError::InsufficientAAmount => RouterErrorsForLibrary::RouterInsufficientAAmount,
            RaumFiRouterError::InsufficientBAmount => RouterErrorsForLibrary::RouterInsufficientBAmount,
            RaumFiRouterError::InsufficientOutputAmount => RouterErrorsForLibrary::RouterInsufficientOutputAmount,
            RaumFiRouterError::ExcessiveInputAmount => RouterErrorsForLibrary::RouterExcessiveInputAmount,
            RaumFiRouterError::PairDoesNotExist => RouterErrorsForLibrary::RouterPairDoesNotExist,
        }
    }
}
