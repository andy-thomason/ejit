pub mod hash {
    //! https://github.com/ethereum/execution-specs/blob/master/src/ethereum/crypto/hash.py
    use crate::ethereum::ethereum_types::bytes::*;

    pub struct Hash32(pub Bytes32);
    pub struct Hash64(pub Bytes64);
    
    
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
    pub fn keccak256(buffer: Bytes) -> Hash32 {
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
    

}
