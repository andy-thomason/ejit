use std::{ops::{Add, Div, Mul, Sub}, process::Output};

use crate::ethereum::exceptions::Exception;

pub type Int = i128;
pub type Uint = u128;
pub type U8 = u8;
pub type U32 = u32;
pub type U64 = u64;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct U256([u64; 4]);

impl U256 {
    pub const ZERO : U256 = U256([0; 4]);

    pub fn from_be_bytes(value: [u8; 32]) -> Self {
        Self::from_limbs([
            u64::from_be_bytes(value[0x00..0x08].try_into().unwrap()),
            u64::from_be_bytes(value[0x08..0x10].try_into().unwrap()),
            u64::from_be_bytes(value[0x10..0x18].try_into().unwrap()),
            u64::from_be_bytes(value[0x18..0x20].try_into().unwrap()),
        ])
    }

    pub const fn to_limbs(&self) -> [u64; 4] {
        self.0
    }

    pub fn to_be_bytes(&self) -> [u8; 32] {
        let [a, b, c, d] = self.to_limbs();
        let mut res = [0; 32];
        res[0x00..0x08].copy_from_slice(&a.to_be_bytes());
        res[0x08..0x10].copy_from_slice(&b.to_be_bytes());
        res[0x10..0x18].copy_from_slice(&c.to_be_bytes());
        res[0x18..0x20].copy_from_slice(&d.to_be_bytes());
        res
    }

    pub const fn from_limbs(value: [u64; 4]) -> Self {
        Self(value)
    }

    pub const fn from_int(value: i32) -> Self {
        let sign = u64::from_be_bytes(((value as i64) << 63 >> 63).to_be_bytes());
        let val = u64::from_be_bytes(((value as i64) << 32 >> 32).to_be_bytes());
        Self::from_limbs([sign, sign, sign, val])
    }

    pub fn is_zero(&self) -> bool {
        (self.0[0] | self.0[1] | self.0[2] | self.0[3]) == 0
    }

    pub fn to_uint(&self) -> Result<Uint, Exception> {
        if self.0[0] != 0 || self.0[1] != 0 {
            return Err(Exception::NumericOverflow);
        }
        Ok(((self.0[2] as u128) << 64) | self.0[3] as u128)
    }
}

impl From<i32> for U256 {
    fn from(value: i32) -> Self {
        let sign = u64::from_be_bytes(((value as i64) << 63 >> 63).to_be_bytes());
        let val = u64::from_be_bytes(((value as i64) << 32 >> 32).to_be_bytes());
        Self::from_limbs([sign, sign, sign, val])
    }
}

impl From<u32> for U256 {
    fn from(value: u32) -> Self {
        Self::from_limbs([0, 0, 0, value as u64])
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        Self::from_limbs([0, 0, 0, value])
    }
}

impl Add<U256> for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> Self::Output {
        let ca = self.to_limbs();
        let cb = rhs.to_limbs();
        let (sum0, cy0) = ca[3].overflowing_add(cb[3]);

        let (sum1, cy1a) = ca[2].overflowing_add(cb[2]);
        let (sum1, cy1b) = sum1.overflowing_add(if cy0 { 1 } else {0} );
    
        let (sum2, cy2a) = ca[1].overflowing_add(cb[1]);
        let (sum2, cy2b) = sum2.overflowing_add(if cy1a || cy1b { 1 } else {0} );
    
        let (sum3, _cy3a) = ca[0].overflowing_add(cb[0]);
        let (sum3, _cy3b) = sum3.overflowing_add(if cy2a || cy2b { 1 } else {0} );
    
        Self::from_limbs([sum0, sum1, sum2, sum3])
    }
}

impl Sub<U256> for U256 {
    type Output = U256;

    fn sub(self, rhs: U256) -> Self::Output {
        let ca = self.to_limbs();
        let cb = rhs.to_limbs();
        let (sum0, cy0) = ca[3].overflowing_sub(cb[3]);

        let (sum1, cy1a) = ca[2].overflowing_sub(cb[2]);
        let (sum1, cy1b) = sum1.overflowing_sub(if cy0 { 1 } else {0} );
    
        let (sum2, cy2a) = ca[1].overflowing_sub(cb[1]);
        let (sum2, cy2b) = sum2.overflowing_sub(if cy1a || cy1b { 1 } else {0} );
    
        let (sum3, _cy3a) = ca[0].overflowing_sub(cb[0]);
        let (sum3, _cy3b) = sum3.overflowing_sub(if cy2a || cy2b { 1 } else {0} );
    
        Self::from_limbs([sum3, sum2, sum1, sum0])
    }
}

impl Mul<U256> for U256 {
    type Output = U256;

    fn mul(self, rhs: U256) -> Self::Output {
        let ca = self.to_limbs();
        let cb = rhs.to_limbs();
        let sum0 =
            ca[3] as u128 * cb[3] as u128
        ;

        let sum1 =
            ca[2] as u128 * cb[3] as u128 +
            ca[3] as u128 * cb[2] as u128 +
            sum0 >> 64
        ;

        let sum2 =
            ca[1] as u128 * cb[3] as u128 +
            ca[2] as u128 * cb[2] as u128 +
            ca[3] as u128 * cb[1] as u128 +
            sum1 >> 64
        ;

        let sum3 =
            ca[0] as u128 * cb[3] as u128 +
            ca[1] as u128 * cb[2] as u128 +
            ca[2] as u128 * cb[1] as u128 +
            ca[3] as u128 * cb[0] as u128 +
            sum2 >> 64
        ;

        fn trunc(x: u128) -> u64 {
            (x & (u64::MAX as u128)) as u64
        }
        Self::from_limbs([trunc(sum3), trunc(sum2), trunc(sum1), trunc(sum0)])
    }
}

impl Div<U256> for U256 {
    type Output = U256;

    fn div(self, mut rhs: U256) -> Self::Output {
        // I know faster methods exist, but you probably shouldn't be
        // using a divide anyway!
        let mut res = Self::ZERO;
        for i in 0..256 {
            res = res.clone() + res;
            if rhs >= self {
                res = res + U256::from(1);
                rhs = rhs - self;
            }
            rhs = rhs.clone() + rhs;
        }

        res
    }
}



#[test]
fn test_u256() {
    assert_eq!(U256::from_int(-1) + U256::from_int(1), U256::from_int(0));
    assert_eq!(U256::from_int(1) + U256::from_int(-1), U256::from_int(0));
}
