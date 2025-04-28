

/// https://github.com/ethereum/ethereum-types/blob/master/src/ethereum_types/numeric.py
pub mod numeric {
    pub type Int = i128;
    pub type Uint = u128;

    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct U256(pub [u8; 32]);
    
    impl From<u32> for U256 {
        fn from(value: u32) -> Self {
            let mut res = [0; 32];
            res[0x1c..0x20].copy_from_slice(&value.to_be_bytes());
            Self(res)
        }
    }

    pub type U8 = u8;
    pub type U32 = u32;
    pub type U64 = u64;

    // #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    // pub struct U8(pub u8);
    // #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    // pub struct U32(pub u32);
    // #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    // pub struct U64(pub u64);
}

/// https://github.com/ethereum/ethereum-types/blob/master/src/ethereum_types/bytes.py
pub mod bytes {
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes0(pub [u8; 0]);

    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes1(pub [u8; 1]);
    
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes4(pub [u8; 4]);
    

    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes8(pub [u8; 8]);
    
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes20(pub [u8; 20]);
    
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes32(pub [u8; 32]);
    
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes48(pub [u8; 48]);
    
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes64(pub [u8; 64]);
    
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes96(pub [u8; 96]);
    
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes256(pub [u8; 256]);
    
    /// Sequence of bytes (octets) of arbitrary length.
    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Bytes(pub Vec<u8>);

    impl std::ops::Deref for Bytes {
        type Target = [u8];
    
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl From<&[u8]> for Bytes {
        fn from(value: &[u8]) -> Self {
            Bytes(value.to_vec())
        }
    }
}
