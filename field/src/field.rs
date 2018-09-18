//! Abstract field

use std::ops::{Add, Mul, Sub, Div};
use arith::Value;

/// Abstract field description
pub trait Field: Copy + Clone + PartialEq + ::std::fmt::Debug {
    /// Type of scalar
    type Value;
    /// Field modulus
    const MODULUS: Self::Value;
    /// Field montgomery coefficient (R)
    const R: Self::Value;
    /// Multiplicative inverse of R modulus MODULUS
    const R_INVERSE: Self::Value;
}

/// Generalization of field element
pub trait FieldValue:
    Sized +
    Clone +
    Copy +
    PartialEq +
    ::std::fmt::Debug +
    Mul<Output=Self> +
    Mul<u32, Output=Self> +
    Add<Output=Self> +
    Div<Output=Self> +
    Sub<Output=Self>
{
    /// Inner scalar value type of field element
    type Value: Value;

    /// Squared field element
    fn squared(self) -> Self {
        self * self
    }

    /// self^other
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