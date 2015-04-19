extern crate num;

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::cmp::{PartialEq, Eq};
use num::{Zero, One};

struct Fixed24p8 {
    base: u32,
}

const NUM_FRAC_BITS: u32 = 8;

impl Add for Fixed24p8 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Fixed24p8 {
            base: self.base + rhs.base
        }
    }
}

impl Sub for Fixed24p8 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Fixed24p8 {
            base: self.base - rhs.base
        }
    }
}

impl Mul for Fixed24p8 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Fixed24p8 {
            base: ((self.base as u64 * rhs.base as u64) >> NUM_FRAC_BITS) as u32,
        }
    }
}

impl Div for Fixed24p8 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Fixed24p8 {
            base: (((self.base as u64) << NUM_FRAC_BITS) / (rhs.base as u64)) as u32,
        }
    }
}

impl Rem for Fixed24p8 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Fixed24p8 {
            base: self.base % rhs.base,
        }
    }
}

impl Zero for Fixed24p8 {
    fn zero() -> Self {
        Fixed24p8 {
            base: 0,
        }
    }

    fn is_zero(&self) -> bool {
        self.base == 0
    }
}

impl One for Fixed24p8 {
    fn one() -> Self {
        Fixed24p8 {
            base: 1 << NUM_FRAC_BITS,
        }
    }
}

impl PartialEq for Fixed24p8 {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl Eq for Fixed24p8 { }
