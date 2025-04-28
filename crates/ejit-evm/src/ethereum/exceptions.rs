//! Error types common across all Ethereum forks.
//! 
//! 

pub enum Exception {
    /// Base class for all exceptions _expected_ to be thrown during normal
    /// operation.
    EthereumException,
    /// Thrown when a block being processed is found to be invalid.
    InvalidBlock(String),
    /// Thrown when a transaction being processed is found to be invalid.
    InvalidTransaction(String),
    /// Thrown when a transaction originates from an account that cannot send
    /// transactions.
    InvalidSenderError(String),
    /// Thrown when a transaction has an invalid signature.
    InvalidSignatureError(String),
}
