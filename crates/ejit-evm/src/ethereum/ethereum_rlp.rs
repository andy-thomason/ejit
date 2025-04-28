pub mod rlp {
    use crate::ethereum::ethereum_types::bytes::Bytes;

    pub trait Extended {
        fn encode(&self) -> Bytes;
    }

    pub fn encode<T : Extended>(t: &T) -> Bytes {
        t.encode()
    }
}

pub mod py {

}

pub mod typed {

}