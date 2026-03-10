use std::fmt;

/// Custom error type for the Lightning UTXO Anchor Manager
#[derive(Debug)]
pub enum ManagerError {
    /// Bitcoin RPC command failed
    RpcError(String),

    /// JSON returned from bitcoin-cli could not be parsed
    JsonParseError(String),

    /// Wallet does not have enough confirmed UTXOs
    InsufficientFunds,

    /// Wallet does not have anchor-capable UTXOs
    NoAnchorCapableUtxos,

    /// Wallet reserve policy violated
    ReserveViolation,

    /// Invalid UTXO detected
    InvalidUtxo,
}

/// Implement Display so errors print nicely in CLI output
impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManagerError::RpcError(msg) => write!(f, "RPC Error: {}", msg),
            ManagerError::JsonParseError(msg) => write!(f, "JSON Parse Error: {}", msg),
            ManagerError::InsufficientFunds => write!(f, "Insufficient wallet funds"),
            ManagerError::NoAnchorCapableUtxos => write!(f, "No anchor-capable UTXOs available"),
            ManagerError::ReserveViolation => write!(f, "Reserve policy violated"),
            ManagerError::InvalidUtxo => write!(f, "Invalid UTXO encountered"),
        }
    }
}

/// Allow ManagerError to behave like a standard Rust error
impl std::error::Error for ManagerError {}
