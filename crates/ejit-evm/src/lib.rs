#![allow(warnings)]
#![doc = include_str!("../README.md")]

use std::collections::BTreeMap;

pub mod ethereum;

pub enum Either<A, B> {
    A(A),
    B(B),
}


// mod ejit_evm;

