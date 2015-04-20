extern crate num;

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::cmp::{PartialEq, Eq};
use num::{Zero, One, Num, FromPrimitive, ToPrimitive, NumCast, Bounded, Saturating};
use num::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};

macro_rules! fixed_point_impl {
    ($name:ident: $ty:ty, $tyd:ty, $ibits:expr, $fbits:expr) => {
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

        impl FromPrimitive for $name {
            fn from_i64(n: i64) -> Option<Self> {
                if n < Self::min_value().to_i64().unwrap() ||
                    n > Self::max_value().to_i64().unwrap() {
                    None
                } else {
                    Some($name {
                        base: n as $ty << $fbits,
                    })
                }
            }

            fn from_u64(n: u64) -> Option<Self> {
                if n > Self::max_value().to_u64().unwrap() {
                    None
                } else {
                    Some($name {
                        base: n as $ty << $fbits,
                    })
                }
            }

            fn from_f64(n: f64) -> Option<Self> {
                if n < Self::min_value().to_f64().unwrap() ||
                    n > Self::max_value().to_f64().unwrap() {
                    None
                } else {
                    Some($name {
                        base: (n * ((1 << $fbits) as f64)) as $ty,
                    })
                }
            }
        }

        impl ToPrimitive for $name {
            fn to_i64(&self) -> Option<i64> {
                Some((self.base >> $fbits) as i64)
            }

            #[allow(unused_comparisons)]
            fn to_u64(&self) -> Option<u64> {
                if self.base < 0 {
                    None
                } else {
                    Some((self.base >> $fbits) as u64)
                }
            }

            fn to_f64(&self) -> Option<f64> {
                Some(self.base as f64 / (1 << $fbits) as f64)
            }
        }

        impl NumCast for $name {
            fn from<T: ToPrimitive>(n: T) -> Option<Self> {
                n.to_f64().and_then(|f| FromPrimitive::from_f64(f))
            }
        }

        impl Bounded for $name {
            fn min_value() -> Self {
                $name {
                    base: Bounded::min_value(),
                }
            }

            fn max_value() -> Self {
                $name {
                    base: Bounded::max_value(),
                }
            }
        }

        impl Saturating for $name {
            fn saturating_add(self, rhs: Self) -> Self {
                $name {
                    base: self.base.saturating_add(rhs.base),
                }
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                $name {
                    base: self.base.saturating_sub(rhs.base),
                }
            }
        }

        impl CheckedAdd for $name {
            fn checked_add(&self, rhs: &Self) -> Option<Self> {
                self.base.checked_add(rhs.base).map(|b| $name {
                    base: b,
                })
            }
        }

        impl CheckedSub for $name {
            fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                self.base.checked_sub(rhs.base).map(|b| $name {
                    base: b,
                })
            }
        }

        impl CheckedMul for $name {
            fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                (self.base as $tyd).checked_mul(rhs.base as $tyd).and_then(|mut base_double| {
                    base_double = base_double >> $fbits;
                    let $name { base: max_base } = Self::max_value();
                    let $name { base: min_base } = Self::min_value();
                    if base_double > max_base as $tyd || base_double < min_base as $tyd {
                        None
                    } else {
                        Some($name {
                            base: base_double as $ty
                        })
                    }
                })
            }
        }

        impl CheckedDiv for $name {
            fn checked_div(&self, rhs: &Self) -> Option<Self> {
                ((self.base as $tyd) << $fbits).checked_div(rhs.base as $tyd)
                    .and_then(|base_double| {
                        let $name { base: max_base } = Self::max_value();
                        let $name { base: min_base } = Self::min_value();
                        if base_double > max_base as $tyd || base_double < min_base as $tyd {
                            None
                        } else {
                            Some($name {
                                base: base_double as $ty
                            })
                        }
                    })
            }
        }
    };
}

fixed_point_impl!(U24p8: u32, u64, 24, 8);
fixed_point_impl!(I24p8: i32, i64, 24, 8);
