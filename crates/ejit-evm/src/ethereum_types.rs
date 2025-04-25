/// https://github.com/ethereum/ethereum-types/blob/master/src/ethereum_types/numeric.py
pub mod numeric {
    pub struct Uint(pub u64);
    pub struct U256(pub [u8; 32]);
    pub struct U8(pub [u8; 8/8]);
    pub struct U32(pub [u8; 32/8]);
    pub struct U64(pub [u8; 64/8]);
}

/// https://github.com/ethereum/ethereum-types/blob/master/src/ethereum_types/bytes.py
pub mod bytes {
    pub struct Bytes0(pub [u8; 0/8]);

    pub struct Bytes1(pub [u8; 1/8]);
    
    pub struct Bytes4(pub [u8; 4/8]);
    
    pub struct Bytes8(pub [u8; 8/8]);
    
    pub struct Bytes20(pub [u8; 20/8]);
    
    pub struct Bytes32(pub [u8; 32/8]);
    
    pub struct Bytes48(pub [u8; 48/8]);
    
    pub struct Bytes64(pub [u8; 64/8]);
    
    pub struct Bytes96(pub [u8; 96/8]);
    
    pub struct Bytes256(pub [u8; 256/8]);
    
    /// Sequence of bytes (octets) of arbitrary length.
    pub struct Bytes(pub Vec<u8>);
}
