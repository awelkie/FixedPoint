extern crate num;

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::cmp::{PartialEq, Eq};
use num::{Zero, One, Num};

macro_rules! fixed_point_impl {
    ($name:ident: $ty:ty, $tyd:ty, $fbits:expr) => {
        struct $name {
            base: $ty,
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                $name {
                    base: self.base + rhs.base
                }
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                $name {
                    base: self.base - rhs.base
                }
            }
        }

        impl Mul for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                $name {
                    base: ((self.base as $tyd * rhs.base as $tyd) >> $fbits) as $ty,
                }
            }
        }

        impl Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                $name {
                    base: (((self.base as $tyd) << $fbits) / (rhs.base as $tyd)) as $ty,
                }
            }
        }

        impl Rem for $name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                $name {
                    base: self.base % rhs.base,
                }
            }
        }

        impl Zero for $name {
            fn zero() -> Self {
                $name {
                    base: 0,
                }
            }

            fn is_zero(&self) -> bool {
                self.base == 0
            }
        }

        impl One for $name {
            fn one() -> Self {
                $name {
                    base: 1 << $fbits,
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.base == other.base
            }
        }

        impl Eq for $name { }

        impl Num for $name {
            type FromStrRadixErr = ();
            fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                unimplemented!();
            }
        }
    };
}

fixed_point_impl!(Fixed24p8: u32, u64, 8);
