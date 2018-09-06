//! Abstract field

use std::ops::{Add, Mul, Neg, Sub, Div};
use arith::{Value, MulScalar};

/// Abstract field description
pub trait Field: Copy + Clone + PartialEq {
    /// Type of scalar
    type Value;
    /// Field modulus
    const MODULUS: Self::Value;
    /// Field montgomery coefficient (R)
    const R: Self::Value;
    /// Multiplicative inverse of R modulus MODULUS
    const R_INVERSE: Self::Value;
}

pub trait FieldValue:
    Sized +
    Copy +
    Clone +
    PartialEq +
    Mul<Output=Self> +
    Add<Output=Self> +
    Div<Output=Self> +
    Sub<Output=Self> +
    MulScalar
{
    type Value: Value;

    fn squared(self) -> Self {
        self * self
    }

    fn pow(self, other: Self::Value) -> Self {
        let mut res = Self::one();

        for i in other.bits() {
            res = res.squared();
            if i {
                res = self * res;
            }
        }

        res
    }

     /// Multiplication identity
    fn one() -> Self;

    /// Additive identity
    fn zero() -> Self;
}