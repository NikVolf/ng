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
    /// Deconstruct and return raw value
    pub fn into_value(self) -> V {
        self.value
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

impl<F: field::Field<Value=V>, V: arith::Value> arith::MulScalar for FieldElement<F, V> {
    fn mul_scalar(self, other: u32) -> Self {
        self.value.mul_scalar(other, F::MODULUS).into()
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

impl<F: field::Field<Value=V>, V: arith::Value> field::FieldValue for FieldElement<F, V> {
    type Value = V;

    /// Multiplication identity
    fn one() -> Self {
        FieldElement {
            value: V::one(),
            field: PhantomData,
        }
    }

    /// Additive identity
    fn zero() -> Self {
        FieldElement {
            value: V::zero(),
            field: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {

    use test::{Mod19Field, Mod1125899839733759Field};
    use super::FieldElement;
    use field::FieldValue;
    use quickcheck::TestResult;

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

     quickcheck! {
         fn number_div_by_self_equals_one(x: u64) -> TestResult {
             if x % 1125899839733759 == 0 {
                 TestResult::discard()
             } else {
                let x_e: FieldElement<Mod1125899839733759Field, _> = (x % 1125899839733759).into();

                TestResult::from_bool(x_e / x_e == 1.into())
             }
         }

         fn one_div_number_equals_inverse(x: u64) -> TestResult {
             use arith::ModuleInv;

             if x % 1125899839733759 == 0 {
                 TestResult::discard()
             } else {
                 let x_e: FieldElement<Mod1125899839733759Field, _> = (x % 1125899839733759).into();

                 TestResult::from_bool(FieldElement::from(1) / x_e == x_e.into_value().inv(1125899839733759).into())
             }
         }
     }
}