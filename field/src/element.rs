//! Abstract field element implementation

use std::ops::{Add, Mul, Neg, Sub};
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
    fn add(mut self, other: FieldElement<F, V>) -> Self::Output {
        ((self.value + other.value) % F::modulus()).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> From<V> for FieldElement<F, V> {
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
        fn modulus() -> u64 {
            17
        }
    }

    #[test]
    fn smoky() {
        let elem1: FieldElement<Mod17Field, u64> = 1.into();
        let elem2: FieldElement<Mod17Field, u64> = 16.into();

        assert_eq!(elem1 + elem2, 0.into());
        assert_eq!(elem2 + elem1, 0.into());
    }
}