#![allow(warnings)]
#![doc = include_str!("../README.md")]

use std::collections::BTreeMap;

pub mod ethereum;

#[derive(Debug, Clone, PartialEq)]
pub enum Either<A : std::fmt::Debug+Clone, B : std::fmt::Debug+Clone> {
    A(A),
    B(B),
}


// mod ejit_evm;

