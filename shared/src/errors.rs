use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    AlreadyInitialized = 2,
    NotInitialized = 3,
    ContractNotFound = 4,
    InvalidInput = 5,
    MaxHistoryExceeded = 6,
    RuleNotFound = 7,
    Paused = 8,
    RegistryCallFailed = 9,
    HealthCallFailed = 10,
}