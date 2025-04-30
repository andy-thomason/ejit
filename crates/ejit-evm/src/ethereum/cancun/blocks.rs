//! A `Block` is a single link in the chain that is Ethereum. Each `Block` contains
//! a `Header` and zero or more transactions. Each `Header` contains associated
//! metadata like the block number, parent block hash, and how much gas was
//! consumed by its transactions.
//!
//! Together, these blocks form a cryptographically secure journal recording the
//! history of all state transitions that have happened since the genesis of the
//! chain.

use crate::{
    ethereum::{
        cancun::fork_types::{Address, Bloom, Root},
        crypto::hash::Hash32,
        ethereum_rlp::{exceptions::RLPException, rlp},
        ethereum_types::{
            bytes::{Bytes, Bytes32, Bytes8},
            numeric::{Uint, U256, U64},
        },
    }, Either
};

use super::transactions::LegacyTransaction;

/// Withdrawals that have been validated on the consensus layer.
pub struct Withdrawal {
    pub index: U64,
    pub validator_index: U64,
    pub address: Address,
    pub amount: U256,
}

/// Header portion of a block on the chain.
pub struct Header {
    pub parent_hash: Hash32,
    pub ommers_hash: Hash32,
    pub coinbase: Address,
    pub state_root: Root,
    pub transactions_root: Root,
    pub receipt_root: Root,
    pub bloom: Bloom,
    pub difficulty: Uint,
    pub number: Uint,
    pub gas_limit: Uint,
    pub gas_used: Uint,
    pub timestamp: U256,
    pub extra_data: Bytes,
    pub prev_randao: Bytes32,
    pub nonce: Bytes8,
    pub base_fee_per_gas: Uint,
    pub withdrawals_root: Root,
    pub blob_gas_used: U64,
    pub excess_blob_gas: U64,
    pub parent_beacon_block_root: Root,
}

impl rlp::Extended for Header {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }

    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

/// A complete block.
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Either<LegacyTransaction, Bytes>>,
    pub ommers: Vec<Header>,
    pub withdrawals: Vec<Withdrawal>,
}

/// Data record produced during the execution of a transaction.
pub struct Log {
    pub address: Address,
    pub topics: Vec<Hash32>,
    pub data: Bytes,
}

/// Result of a transaction.
pub struct Receipt {
    pub succeeded: bool,
    pub cumulative_gas_used: Uint,
    pub bloom: Bloom,
    pub logs: Vec<Log>,
}
