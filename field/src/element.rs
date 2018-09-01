//! Abstract field element implementation

use std::ops::{Add, Mul, Neg, Sub, Div};
use std::marker::PhantomData;

use {field, arith};

/// Field element on the field F with value V
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FieldElement<F: field::Field<Value=V>, V: arith::Value> {
    value: V,
    field: PhantomData<F>,
}

impl<F: field::Field<Value=V>, V: arith::Value> FieldElement<F, V> {
    /// Multiplication identity
    pub fn one() -> Self {
        FieldElement {
            value: V::one(),
            field: PhantomData,
        }
    }

    /// Additive identity
    pub fn zero() -> Self {
        FieldElement {
            value: V::zero(),
            field: PhantomData,
        }
    }

    /// Deconstruct and return raw value
    pub fn into_value(self) -> V {
        self.value
    }

    /// Squared field element
    pub fn squared(self) -> Self {
        self * self
    }

    /// self ** other
    pub fn pow(self, other: V) -> Self {
        let mut res = Self::one();

        for i in other.bits() {
            res = res.squared();
            if i {
                res = self * res;
            }
        }

        res
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Add for FieldElement<F, V> {
    type Output = FieldElement<F, V>;
    fn add(self, other: FieldElement<F, V>) -> Self::Output {
        self.value.add(other.value, F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Sub for FieldElement<F, V> {
    type Output = FieldElement<F, V>;
    fn sub(self, other: FieldElement<F, V>) -> Self::Output {
        self.value.add(other.value.neg(F::MODULUS), F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Neg for FieldElement<F, V> {
    type Output = FieldElement<F, V>;
    fn neg(self) -> Self::Output {
        self.value.neg(F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Mul for FieldElement<F, V> {
    type Output = FieldElement<F, V>;
    fn mul(self, other: Self) -> Self::Output {
        self.value.mul(other.value, F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Div for FieldElement<F, V> {
    type Output = FieldElement<F, V>;
    fn div(self, other: Self) -> Self::Output {
        self.value.mul(other.value.inv(F::MODULUS), F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> From<V> for FieldElement<F, V>
{
    fn from(val: V) -> Self {
        FieldElement {
            value: val,
            field: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {

    use test::Mod19Field;
    use super::FieldElement;

    #[test]
    fn smoky() {
        let elem1: FieldElement<Mod19Field, _> = 6.into();
        let elem2: FieldElement<Mod19Field, _> = 16.into();

        assert_eq!(elem1 + elem2, 3.into());
        assert_eq!(elem2 + elem1, 3.into());

        assert_eq!(elem1 - elem2, 9.into());
        assert_eq!(elem2 - elem1, 10.into());

        assert_eq!(-elem1, 13.into());
        assert_eq!(-elem2, 3.into());

        assert_eq!(elem1*elem2, 1.into());
        assert_eq!(elem2*elem1, 1.into());

        assert_eq!(elem2/elem1, 9.into());
        assert_eq!(elem1/elem2, 17.into());

        assert_eq!(elem1.pow(20), 17.into());
        assert_eq!(elem2.pow(10), 16.into());
     }
}