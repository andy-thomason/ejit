use crate::crypto::hash::Hash32;
use crate::ethereum_types::bytes::*;
use crate::ethereum_types::numeric::*;

pub struct Address(pub Bytes20);
pub struct Root(pub Hash32);
pub struct VersionedHash(pub Hash32);

pub struct Bloom(pub Bytes256);


/// State associated with an address.
pub struct Account {
    nonce: Uint,
    balance: U256,
    code: Bytes,
}

pub const EMPTY_ACCOUNT : Account = Account{
    nonce: Uint(0),
    balance: U256([0; 32]),
    code: Bytes(Vec::new()),
};


/// Encode `Account` dataclass.
/// 
/// Storage is not stored in the `Account` dataclass, so `Accounts` cannot be
/// encoded without providing a storage root.
pub fn encode_account(raw_account_data: Account, storage_root: Bytes) -> Bytes {
    // rlp.encode(
    //     (
    //         raw_account_data.nonce,
    //         raw_account_data.balance,
    //         storage_root,
    //         keccak256(raw_account_data.code),
    //     )
    // )
    todo!();
}
