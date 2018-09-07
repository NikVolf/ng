//! Abstract montgomery field element

use std::ops::{Add, Mul, Neg, Sub, Div};
use std::marker::PhantomData;

use {field, arith, element};

/// Field element on the field F with value V in montgomery representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MontgomeryElement<F: field::Field<Value=V>, V: arith::Value> {
    value: V,
    field: PhantomData<F>,
}

impl<F: field::Field<Value=V>, V: arith::Value> field::FieldValue for MontgomeryElement<F, V> {
    type Value = V;

    /// Multiplication identity
    fn one() -> Self {
        MontgomeryElement {
            value: F::R,
            field: PhantomData,
        }
    }

    /// Additive identity
    fn zero() -> Self {
        MontgomeryElement {
            value: V::zero(),
            field: PhantomData,
        }
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> MontgomeryElement<F, V> {
    /// New field element from regular form
    pub fn from_element(t: element::FieldElement<F, V>) -> Self {
        t.into_value().into()
    }

    /// Convert to regular form
    pub fn into_element(self) -> element::FieldElement<F, V> {
        self.value.mul(F::R_INVERSE, F::MODULUS).into()
    }

    /// Deconstruct and return raw value (not reduced)
    pub fn into_value(self) -> V {
        self.value
    }

    /// Construct from raw value (should be reduced in advance)
    pub(crate) fn from_raw(val: V) -> Self {
        MontgomeryElement {
            value: val,
            field: PhantomData,
        }
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Add for MontgomeryElement<F, V> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.add(other.value, F::MODULUS))
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Sub for MontgomeryElement<F, V> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.add(other.value.neg(F::MODULUS), F::MODULUS))
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Neg for MontgomeryElement<F, V> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.neg(F::MODULUS))
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Mul for MontgomeryElement<F, V> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.mul_reduce(other.value, F::MODULUS, F::R_INVERSE))
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> arith::MulScalar for MontgomeryElement<F, V> {
    fn mul_scalar(self, other: u32) -> Self {
        self.value.mul_scalar(other, F::MODULUS).into()
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> Div for MontgomeryElement<F, V> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.mul_reduce(other.value.inv(F::MODULUS), F::MODULUS, F::R))
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> From<V> for MontgomeryElement<F, V>
{
    fn from(val: V) -> Self {
        MontgomeryElement {
            value: val.mul(F::R, F::MODULUS),
            field: PhantomData,
        }
    }
}


#[cfg(test)]
mod tests {

    use {MontgomeryElement, FieldValue};
    use test::Mod19Field;

    #[test]
    fn smoky() {
        let elem1: MontgomeryElement<Mod19Field, _> = 6.into();
        assert_eq!(elem1.into_value(), 1);

        let elem2: MontgomeryElement<Mod19Field, _> = 16.into();
        assert_eq!(elem2.into_value(), 9);

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