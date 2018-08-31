//! Abstract field element implementation

use std::ops::{Add, Mul, Neg, Sub, Div};
use std::marker::PhantomData;

use {field, arith};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FieldElement<F: field::Field<Value=V>, V: arith::Value> {
    value: V,
    field: PhantomData<F>,
}

impl<F: field::Field<Value=V>, V: arith::Value> FieldElement<F, V> {
    pub fn one() -> Self {
        FieldElement {
            value: V::one(),
            field: PhantomData,
        }
    }

    pub fn zero() -> Self {
        FieldElement {
            value: V::zero(),
            field: PhantomData,
        }
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

    use {arith, field};
    use super::FieldElement;

    impl arith::Value for u64 {
        fn one() -> u64 {
            1
        }
        fn zero() -> u64 {
            0
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Mod17Field;

    impl field::Field for Mod17Field {
        type Value = u64;
        const MODULUS: Self::Value = 17;
    }

    #[test]
    fn smoky() {
        let elem1: FieldElement<Mod17Field, u64> = 6.into();
        let elem2: FieldElement<Mod17Field, u64> = 16.into();

        assert_eq!(elem1 + elem2, 5.into());
        assert_eq!(elem2 + elem1, 5.into());
        assert_eq!(elem1 - elem2, 7.into());
        assert_eq!(elem2 - elem1, 10.into());
        assert_eq!(-elem1, 11.into());
        assert_eq!(-elem2, 1.into());
        assert_eq!(elem1*elem2, 11.into());
        assert_eq!(elem2*elem1, 11.into());
        assert_eq!(elem2/elem1, 14.into());
        assert_eq!(elem1/elem2, 11.into());
    }
}