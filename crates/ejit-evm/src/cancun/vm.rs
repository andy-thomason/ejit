//! https://github.com/ethereum/execution-specs/blob/master/src/ethereum/cancun/vm/__init__.py
//! 

use std::collections::BTreeMap;

use crate::{crypto::hash::Hash32, ethereum_types::{bytes::*, numeric::*}, fork_types::*};

pub mod gas;

struct Dict(BTreeMap<u64, u64>);

/// Items external to the virtual machine itself, provided by the environment.
pub struct Environment {
    caller: Address,
    block_hashes: Vec<Hash32>,
    origin: Address,
    coinbase: Address,
    number: Uint,
    base_fee_per_gas: Uint,
    gas_limit: Uint,
    gas_price: Uint,
    time: U256,
    prev_randao: Bytes32,
    state: State,
    chain_id: U64,
    traces: Vec<Dict>,
    excess_blob_gas: U64,
    blob_versioned_hashes: Vec<VersionedHash>,
    transient_storage: TransientStorage,
}

pub enum Either<A, B> {
    A(A),
    B(B),
}

/// Items that are used by contract creation or message call.
pub struct Message {
    caller: Address,
    target: Either<Bytes0, Address>,
    current_target: Address,
    gas: Uint,
    value: U256,
    data: Bytes,
    code_address: Option<Address>,
    code: Bytes,
    depth: Uint,
    should_transfer_value: bool,
    is_static: bool,
    accessed_addresses: Vec<Address>,
    accessed_storage_keys: Vec<(Address, Bytes32)>,
    parent_evm: Option<Box<Evm>>,
}


/// The internal state of the virtual machine.
pub struct Evm {
    pc: Uint,
    stack: Vec<U256>,
    memory: Vec<u8>,
    code: Bytes,
    gas_left: Uint,
    env: Environment,
    valid_jump_destinations: Vec<Uint>,
    logs: Vec<Log>,
    refund_counter: i64,
    running: bool,
    message: Message,
    output: Bytes,
    accounts_to_delete: Vec<Address>,
    touched_accounts: Vec<Address>,
    return_data: Bytes,
    error: Optional<EthereumException>,
    accessed_addresses: Vec<Address>,
    accessed_storage_keys: Vec<(Address, Bytes32)>,
}
