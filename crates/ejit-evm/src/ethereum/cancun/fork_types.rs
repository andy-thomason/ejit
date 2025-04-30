use std::ops::Deref;

use crate::ethereum::{crypto::hash::Hash32, ethereum_types::{bytes::{Bytes20, Bytes256, *}, numeric::*}};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Address([u8; 20]);

impl Address {
    pub const fn from_be_bytes(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl Deref for Address {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; 20]> for Address {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Root(pub Hash32);

impl Deref for Root {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0.0.0
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct VersionedHash(pub Hash32);

impl Deref for VersionedHash {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0.0.0
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bloom(pub Bytes256);

impl Deref for Bloom {
    type Target = [u8; 256];

    fn deref(&self) -> &Self::Target {
        &self.0.0
    }
}

/// State associated with an address.
pub struct Account {
    pub nonce: Uint,
    pub balance: U256,
    pub code: Bytes,
}

pub static EMPTY_ACCOUNT : Account = Account{
    nonce: 0,
    balance: U256::ZERO,
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
