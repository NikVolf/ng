//! Abstract montgomery field element

use std::ops::{Add, Mul, Neg, Sub};
use std::marker::PhantomData;

use {field, arith, element};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MontgomeryElement<F: field::Field<Value=V>, V: arith::Value> {
    value: V,
    field: PhantomData<F>,
}

impl<F: field::Field<Value=V>, V: arith::Value> MontgomeryElement<F, V> {
    pub fn one() -> Self {
        MontgomeryElement {
            value: V::one(),
            field: PhantomData,
        }
    }

    pub fn zero() -> Self {
        MontgomeryElement {
            value: V::zero(),
            field: PhantomData,
        }
    }

    pub fn from_element(t: element::FieldElement<F, V>) -> Self {
        MontgomeryElement {
            value: t.into_value().mul(F::R, F::MODULUS),
            field: PhantomData,
        }
    }

    pub fn into_element(self) -> element::FieldElement<F, V> {
        self.value.mul(F::R_INVERSE, F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Add for MontgomeryElement<F, V> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.value.add(other.value, F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Sub for MontgomeryElement<F, V> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.value.add(other.value.neg(F::MODULUS), F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Neg for MontgomeryElement<F, V> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.value.neg(F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Mul for MontgomeryElement<F, V> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        self.value.mul_reduce(other.value, F::MODULUS, F::R_INVERSE).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> From<V> for MontgomeryElement<F, V>
{
    fn from(val: V) -> Self {
        MontgomeryElement {
            value: val,
            field: PhantomData,
        }
    }
}