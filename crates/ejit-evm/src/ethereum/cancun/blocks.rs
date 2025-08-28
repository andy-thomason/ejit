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
        ethereum_rlp::{exceptions::RLPException, rlp::{self, decode_to_sequence, encode_sequence, Extended}},
        ethereum_types::{
            bytes::{Bytes, Bytes32, Bytes8},
            numeric::{Uint, U256, U64},
        },
    }, impl_extended
};

use super::transactions::{LegacyTransaction, Transaction};

#[derive(Debug, Clone, Default)]
/// Withdrawals that have been validated on the consensus layer.
pub struct Withdrawal {
    pub index: U64,
    pub validator_index: U64,
    pub address: Address,
    pub amount: U256,
}

impl_extended!(Withdrawal: index, validator_index, address, amount);

// impl Extended for Withdrawal {
//     fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
//         encode_sequence(buffer, &[
//             &self.index,
//             &self.validator_index,
//             &self.address,
//             &self.amount,
//         ])
//     }

//     fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
//         decode_to_sequence(buffer, &mut [
//             &mut self.index,
//             &mut self.validator_index,
//             &mut self.address,
//             &mut self.amount,
//         ])
//     }
// }

#[derive(Debug, Clone, Default)]
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
    pub base_fee_per_gas: Option<Uint>,
    pub withdrawals_root: Option<Root>,
    pub blob_gas_used: Option<U64>,
    pub excess_blob_gas: Option<U64>,
    pub parent_beacon_block_root: Option<Root>,
}

impl_extended!(Header: parent_hash, ommers_hash, coinbase, state_root, transactions_root, receipt_root, bloom, difficulty, number, gas_limit, gas_used, timestamp, extra_data, prev_randao, nonce, base_fee_per_gas, withdrawals_root, blob_gas_used, excess_blob_gas, parent_beacon_block_root);

// impl rlp::Extended for Header {
//     fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
//         encode_sequence(buffer, &[
//             &self.parent_hash,
//             &self.ommers_hash,
//             &self.coinbase,
//             &self.state_root,
//             &self.transactions_root,
//             &self.receipt_root,
//             &self.bloom,
//             &self.difficulty,
//             &self.number,
//             &self.gas_limit,
//             &self.gas_used,
//             &self.timestamp,
//             &self.extra_data,
//             &self.prev_randao,
//             &self.nonce,
//             &self.base_fee_per_gas,
//             &self.withdrawals_root,
//             &self.blob_gas_used,
//             &self.excess_blob_gas,
//             &self.parent_beacon_block_root,
//         ])
//     }

//     fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
//         decode_to_sequence(buffer, &mut [
//             &mut self.parent_hash,
//             &mut self.ommers_hash,
//             &mut self.coinbase,
//             &mut self.state_root,
//             &mut self.transactions_root,
//             &mut self.receipt_root,
//             &mut self.bloom,
//             &mut self.difficulty,
//             &mut self.number,
//             &mut self.gas_limit,
//             &mut self.gas_used,
//             &mut self.timestamp,
//             &mut self.extra_data,
//             &mut self.prev_randao,
//             &mut self.nonce,
//             &mut self.base_fee_per_gas,
//             &mut self.withdrawals_root,
//             &mut self.blob_gas_used,
//             &mut self.excess_blob_gas,
//             &mut self.parent_beacon_block_root,
//         ])
//     }
// }

#[derive(Debug, Clone, Default)]
/// A complete block.
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
    pub ommers: Vec<Header>,
    pub withdrawals: Option<Vec<Withdrawal>>,
}

// impl Extended for Block {
//     fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
//         encode_sequence(buffer, &[
//             &self.header,
//             &self.transactions,
//             &self.ommers,
//             &self.withdrawals,
//         ])
//     }

//     fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
//         decode_to_sequence(buffer, &mut [
//             &mut self.header,
//             &mut self.transactions,
//             &mut self.ommers,
//             &mut self.withdrawals,
//         ])
//     }
// }

impl_extended!(Block: header,transactions,ommers,withdrawals);

#[derive(Debug, Clone, Default)]
/// Data record produced during the execution of a transaction.
pub struct Log {
    pub address: Address,
    pub topics: Vec<Hash32>,
    pub data: Bytes,
}

impl_extended!(Log: address, topics, data);

// impl Extended for Log {
//     fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
//         encode_sequence(buffer, &[
//             &self.address,
//             &self.topics,
//             &self.data,
//         ])
//     }

//     fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
//         decode_to_sequence(buffer, &mut [
//             &mut self.address,
//             &mut self.topics,
//             &mut self.data,
//         ])
//     }
// }

#[derive(Debug, Clone, Default)]
/// Result of a transaction.
pub struct Receipt {
    pub succeeded: bool,
    pub cumulative_gas_used: Uint,
    pub bloom: Bloom,
    pub logs: Vec<Log>,
}

impl_extended!(Receipt: succeeded, cumulative_gas_used, bloom, logs);

// impl Extended for Receipt {
//     fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
//         encode_sequence(buffer, &[
//             &self.succeeded,
//             &self.cumulative_gas_used,
//             &self.bloom,
//             &self.logs,
//         ])
//     }

//     fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
//         decode_to_sequence(buffer, &mut [
//             &mut self.succeeded,
//             &mut self.cumulative_gas_used,
//             &mut self.bloom,
//             &mut self.logs,
//         ])
//     }
// }

