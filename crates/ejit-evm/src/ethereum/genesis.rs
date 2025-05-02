//! Types and functions for beginning a new chain.
//! 
//! _Genesis_ is the term for the beginning of a new chain, and so a genesis block
//! is a block with no parent (its [`parent_hash`] is all zeros.)
//! 
//! The genesis configuration for a chain is specified with a
//! [`GenesisConfiguration`], and genesis blocks are created with
//! [`add_genesis_block`].
//! 
//! [`parent_hash`]: ref:ethereum.frontier.blocks.Header.parent_hash
//! [`GenesisConfiguration`]: ref:ethereum.genesis.GenesisConfiguration
//! [`add_genesis_block`]: ref:ethereum.genesis.add_genesis_block

use serde_json::Value;

use super::{cancun, ethereum_types::{bytes::{Bytes, Bytes8}, numeric::{Uint, U256, U64}}, exceptions::Exception, utils::hexadecimal::{hex_to_bytes, hex_to_bytes8, hex_to_u256, hex_to_uint}};

/// Configuration for the first block of an Ethereum chain.
/// 
/// Specifies the allocation of ether set out in the pre-sale, and some of
/// the fields of the genesis block.
struct GenesisConfiguration {
    /// Discriminant between diverged blockchains; `1` for Ethereum's main network.
    chain_id: U64,

    /// 
    /// See [`difficulty`] (and subsequent forks.)
    /// 
    /// [`difficulty`]: ref:ethereum.frontier.blocks.Header.difficulty
    difficulty: Uint,

    /// See [`extra_data`] (and subsequent forks.)
    /// 
    /// [`extra_data`]: ref:ethereum.frontier.blocks.Header.extra_data
    extra_data: Bytes,

    /// See [`gas_limit`] (and subsequent forks.)
    /// 
    /// [`gas_limit`]: ref:ethereum.frontier.blocks.Header.gas_limit
    gas_limit: Uint,

    /// See [`nonce`] (and subsequent forks.)
    /// 
    /// [`nonce`]: ref:ethereum.frontier.blocks.Header.nonce
    nonce: Bytes8,

    // See [`timestamp`] (and subsequent forks.)
    // 
    // [`timestamp`]: ref:ethereum.frontier.blocks.Header.timestamp
    timestamp: U256,

    /// State of the blockchain at genesis.
    initial_accounts: Value,
}

/// Read a genesis configuration from the given JSON file path.
/// 
fn get_genesis_configuration(genesis_file: &str) -> Result<GenesisConfiguration, Exception> {
    let text = std::fs::read_to_string(genesis_file)
        .map_err(|_| Exception::EthereumException("unable to read genesis file"))?;

    let genesis_data : Value = serde_json::from_str(&text)
        .map_err(|_| Exception::EthereumException("genesis file not json"))?;

    return Ok(GenesisConfiguration{
        chain_id: genesis_data["config"]["chainId"].as_u64().unwrap(),
        difficulty: hex_to_uint(genesis_data["difficulty"].as_str().unwrap())?,
        extra_data: hex_to_bytes(genesis_data["extraData"].as_str().unwrap())?,
        gas_limit: hex_to_uint(genesis_data["gasLimit"].as_str().unwrap())?,
        nonce: hex_to_bytes8(genesis_data["nonce"].as_str().unwrap())?,
        timestamp: hex_or_base_10_str_to_u256(genesis_data["timestamp"].as_str().unwrap())?,
        initial_accounts: genesis_data["alloc"].clone(),
    })
}

/// Convert a string in either hexadecimal or base-10 to a `U256`.
fn hex_or_base_10_str_to_u256(balance: &str) -> Result<U256, Exception> {
    if balance.starts_with("0x") {
        hex_to_u256(balance)
    } else {
        use std::str::FromStr;
        Ok(U256::from_uint(
            Uint::from_str(balance)
                .map_err(|_| Exception::EthereumException("bad number"))?
        ))
    }
}


// struct GenesisFork {

// }
//     """
//     Pointers to the various types and functions required to build a genesis
//     block.
//     """

//     Address: Type[FixedBytes]
//     Account: Callable[[Uint, U256, bytes], AccountT]
//     Trie: Callable[[bool, object], TrieT]
//     Bloom: Type[FixedBytes]
//     Header: Type[HeaderT]
//     Block: Type[BlockT]
//     hex_to_address: Callable[[str], AddressT]
//     set_account: Callable[[StateT, AddressT, AccountT], object]
//     set_storage: Callable[[StateT, AddressT, Bytes32, U256], object]
//     state_root: Callable[[StateT], Hash32]
//     root: Callable[[TrieT], object]


/// Adds the genesis block to an empty blockchain.
/// 
/// The genesis block is an entirely sui generis block (unique) that is not
/// governed by the general rules applying to all other Ethereum blocks.
/// Instead, the only consensus requirement is that it must be identical to
/// the block added by this function.
/// 
/// The mainnet genesis configuration was originally created using the
/// `mk_genesis_block.py` script. It is long since defunct, but is still
/// available at <https://github.com/ethereum/genesis_block_generator>.
/// 
/// The initial state is populated with balances based on the Ethereum presale
/// that happened on the Bitcoin blockchain. Additional ether worth 1.98% of
/// the presale was given to the foundation.
/// 
/// The `state_root` is set to the root of the initial state. The `gas_limit`
/// and `difficulty` are set to suitable starting values. In particular the
/// low gas limit made sending transactions impossible in the early stages of
/// Frontier.
/// 
/// The `nonce` field is `0x42` referencing Douglas Adams' "HitchHiker's Guide
/// to the Galaxy".
/// 
/// The `extra_data` field contains the hash of block `1028201` on
/// the pre-launch Olympus testnet. The creation of block `1028201` on Olympus
/// marked the "starting gun" for Ethereum block creation. Including its hash
/// in the genesis block ensured a fair launch of the Ethereum mining process.
/// 
/// The remaining fields are set to appropriate default values.
/// 
/// On testnets the genesis configuration usually allocates 1 wei to addresses
/// `0x00` to `0xFF` to avoid edge cases around precompiles being created or
/// cleared (by [EIP-161]).
/// 
/// [EIP-161]: https://eips.ethereum.org/EIPS/eip-161
fn add_genesis_block(
    chain: cancun::fork::BlockChain,
    genesis: GenesisConfiguration,
) {
    // for hex_address, account in genesis.initial_accounts.items():
    //     address = hardfork.hex_to_address(hex_address)
    //     hardfork.set_account(
    //         chain.state,
    //         address,
    //         hardfork.Account(
    //             Uint(int(account.get("nonce", "0"))),
    //             hex_or_base_10_str_to_u256(account.get("balance", 0)),
    //             hex_to_bytes(account.get("code", "0x")),
    //         ),
    //     )
    //     for key, value in account.get("storage", {}).items():
    //         hardfork.set_storage(
    //             chain.state, address, hex_to_bytes32(key), hex_to_u256(value)
    //         )

    // fields = {
    //     "parent_hash": Hash32(b"\0" * 32),
    //     "ommers_hash": keccak256(rlp.encode(())),
    //     "coinbase": Address(b"\0" * Address.LENGTH),
    //     "state_root": hardfork.state_root(chain.state),
    //     "transactions_root": hardfork.root(hardfork.Trie(False, None)),
    //     "receipt_root": hardfork.root(hardfork.Trie(False, None)),
    //     "bloom": hardfork.Bloom(b"\0" * 256),
    //     "difficulty": genesis.difficulty,
    //     "number": Uint(0),
    //     "gas_limit": genesis.gas_limit,
    //     "gas_used": Uint(0),
    //     "timestamp": genesis.timestamp,
    //     "extra_data": genesis.extra_data,
    //     "nonce": genesis.nonce,
    // }

    // if has_field(hardfork.Header, "mix_digest"):
    //     fields["mix_digest"] = Hash32(b"\0" * 32)
    // else:
    //     fields["prev_randao"] = Hash32(b"\0" * 32)

    // if has_field(hardfork.Header, "base_fee_per_gas"):
    //     fields["base_fee_per_gas"] = Uint(10**9)

    // if has_field(hardfork.Header, "withdrawals_root"):
    //     fields["withdrawals_root"] = hardfork.root(hardfork.Trie(False, None))

    // if has_field(hardfork.Header, "blob_gas_used"):
    //     fields["blob_gas_used"] = U64(0)

    // if has_field(hardfork.Header, "excess_blob_gas"):
    //     fields["excess_blob_gas"] = U64(0)

    // if has_field(hardfork.Header, "parent_beacon_block_root"):
    //     fields["parent_beacon_block_root"] = Hash32(b"\0" * 32)

    // genesis_header = hardfork.Header(**fields)

    // block_fields = {
    //     "header": genesis_header,
    //     "transactions": (),
    //     "ommers": (),
    // }

    // if has_field(hardfork.Block, "withdrawals"):
    //     block_fields["withdrawals"] = ()

    // genesis_block = hardfork.Block(**block_fields)

    // chain.blocks.append(genesis_block)
    // chain.chain_id = genesis.chain_id
}
