//! https://github.com/ethereum/execution-specs/blob/master/src/ethereum/crypto/hash.py

use crate::ethereum::ethereum_types::bytes::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Hash32(pub Bytes32);

#[derive(Debug, Clone)]
pub struct Hash64(pub Bytes64);

impl std::ops::Deref for Hash32 {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0.0
    }
}

impl std::ops::Deref for Hash64 {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.0.0
    }
}

/// Computes the keccak256 hash of the input `buffer`.
///
/// Parameters
/// ----------
/// buffer :
///     Input for the hashing function.
///
/// Returns
/// -------
/// hash : `ethereum.base_types.Hash32`
///     Output of the hash function.
pub fn keccak256(buffer: &[u8]) -> Hash32 {
    // k = keccak.new(digest_bits=256)
    // return Hash32(k.update(buffer).digest())
    todo!();
}

/// Computes the keccak512 hash of the input `buffer`.
///
/// Parameters
/// ----------
/// buffer :
///     Input for the hashing function.
///
/// Returns
/// -------
/// hash : `ethereum.base_types.Hash32`
///     Output of the hash function.
fn keccak512(buffer: Bytes) -> Hash64 {
    // k = keccak.new(digest_bits=512)
    // return Hash64(k.update(buffer).digest())
    todo!();
}
