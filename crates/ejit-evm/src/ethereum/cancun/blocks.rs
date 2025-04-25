//! A `Block` is a single link in the chain that is Ethereum. Each `Block` contains
//! a `Header` and zero or more transactions. Each `Header` contains associated
//! metadata like the block number, parent block hash, and how much gas was
//! consumed by its transactions.
//!
//! Together, these blocks form a cryptographically secure journal recording the
//! history of all state transitions that have happened since the genesis of the
//! chain.

use crate::{ethereum::crypto::hash::Hash32, ethereum::ethereum_types::{bytes::{Bytes, Bytes32, Bytes8}, numeric::{Uint, U256, U64}}, ethereum::cancun::fork_types::{Address, Bloom, Root}, Either};

use super::transactions::LegacyTransaction;

/// Withdrawals that have been validated on the consensus layer.
pub struct Withdrawal {
    index: U64,
    validator_index: U64,
    address: Address,
    amount: U256,
}

/// Header portion of a block on the chain.
pub struct Header {
    parent_hash: Hash32,
    ommers_hash: Hash32,
    coinbase: Address,
    state_root: Root,
    transactions_root: Root,
    receipt_root: Root,
    bloom: Bloom,
    difficulty: Uint,
    number: Uint,
    gas_limit: Uint,
    gas_used: Uint,
    timestamp: U256,
    extra_data: Bytes,
    prev_randao: Bytes32,
    nonce: Bytes8,
    base_fee_per_gas: Uint,
    withdrawals_root: Root,
    blob_gas_used: U64,
    excess_blob_gas: U64,
    parent_beacon_block_root: Root,
}


/// A complete block.
pub struct Block {
    header: Header,
    transactions: Vec<Either<Bytes, LegacyTransaction>>,
    ommers: Vec<Header>,
    withdrawals: Vec<Withdrawal>,
}

/// Data record produced during the execution of a transaction.
pub struct Log {
    address: Address,
    topics: Vec<Hash32>,
    data: Bytes,
}

/// Result of a transaction.
pub struct Receipt {
    succeeded: bool,
    cumulative_gas_used: Uint,
    bloom: Bloom,
    logs: Vec<Log>,
}
