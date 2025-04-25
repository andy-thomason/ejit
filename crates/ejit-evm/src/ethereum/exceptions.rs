//! Error types common across all Ethereum forks.


pub struct Exception;

/// Base class for all exceptions _expected_ to be thrown during normal
/// operation.
pub struct EthereumException(Exception);


/// Thrown when a block being processed is found to be invalid.

pub struct InvalidBlock(EthereumException);

/// Thrown when a transaction being processed is found to be invalid.

pub struct InvalidTransaction(EthereumException);

/// Thrown when a transaction originates from an account that cannot send
/// transactions.
pub struct InvalidSenderError(InvalidTransaction);


/// Thrown when a transaction has an invalid signature.
pub struct InvalidSignatureError(InvalidTransaction);
