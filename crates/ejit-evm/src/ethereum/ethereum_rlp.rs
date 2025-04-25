pub mod rlp {
    use crate::ethereum::ethereum_types::bytes::Bytes;

    pub trait Extended {
        fn encode(&self) -> Bytes;
    }
}

pub mod py {

}

pub mod typed {

}