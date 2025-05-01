#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes0(pub [u8; 0]);

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes1(pub [u8; 1]);

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes4(pub [u8; 4]);


#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes8(pub [u8; 8]);

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes20(pub [u8; 20]);

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes32(pub [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes48(pub [u8; 48]);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes64(pub [u8; 64]);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes96(pub [u8; 96]);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes256(pub [u8; 256]);

/// Sequence of bytes (octets) of arbitrary length.
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bytes(pub Vec<u8>);

impl std::ops::Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::ops::DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Bytes(value.to_vec())
    }
}

impl Bytes {
    pub fn push(&mut self, value: u8) {
        self.0.push(value);
    }

    pub fn extend<T : IntoIterator<Item=u8>>(&mut self, value: T) {
        self.0.extend(value);
    }
}
