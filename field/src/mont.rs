//! Abstract montgomery field element

use std::ops::{Add, Mul, Neg, Sub, Div};

use arith::{self, Value, ModAdd, ModMul, ModNeg, ModInv, MulReduce};
use {field, element};

/// Field element on the field F with value V in montgomery representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MontgomeryElement<F: field::Field> {
    value: F::Value,
}

impl<F: field::Field> field::FieldValue for MontgomeryElement<F> {
    type Value = F::Value;

    /// Multiplication identity
    fn one() -> Self {
        MontgomeryElement {
            value: F::R,
        }
    }

    /// Additive identity
    fn zero() -> Self {
        MontgomeryElement {
            value: F::Value::zero(),
        }
    }
}

impl<F: field::Field> MontgomeryElement<F> {
    /// New field element from regular form
    pub fn from_element(t: element::FieldElement<F>) -> Self {
        t.into_value().into()
    }

    /// Convert to regular form
    pub fn into_element(self) -> element::FieldElement<F> {
        self.into_reduced_value().into()
    }

    /// Deconstruct and return raw value (not reduced)
    pub fn into_value(self) -> F::Value {
        self.value
    }

    /// Deconstruct and return raw value (not reduced)
    pub fn into_reduced_value(self) -> F::Value {
        ModMul::<F::Value>::mul(self.value, F::R_INVERSE, F::MODULUS)
    }

    /// Construct from raw value (should be reduced in advance)
    pub(crate) fn from_raw(val: F::Value) -> Self {
        MontgomeryElement {
            value: val,
        }
    }
}

impl<F: field::Field> Add for MontgomeryElement<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.add(other.value, F::MODULUS))
    }
}

impl<F: field::Field> Sub for MontgomeryElement<F> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.add(other.value.neg(F::MODULUS), F::MODULUS))
    }
}

impl<F: field::Field> Neg for MontgomeryElement<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.neg(F::MODULUS))
    }
}

impl<F: field::Field> Mul for MontgomeryElement<F> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.mul_reduce(other.value, F::MODULUS, F::R_INVERSE))
    }
}

impl<F: field::Field> Mul<u32> for MontgomeryElement<F> {
    type Output = Self;
    fn mul(self, other: u32) -> Self {
        MontgomeryElement::from_raw(self.value.mul(other, F::MODULUS))
    }
}

impl<F: field::Field> Div for MontgomeryElement<F> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        MontgomeryElement::from_raw(self.value.mul_reduce(other.value.inv(F::MODULUS), F::MODULUS, F::R))
    }
}

impl<F: field::Field<Value=V>, V: arith::Value> From<V> for MontgomeryElement<F>
{
    fn from(val: V) -> Self {
        MontgomeryElement {
            value: val.mul(F::R, F::MODULUS),
        }
    }
}


#[cfg(test)]
mod tests {

    use {MontgomeryElement, FieldValue};
    use test::{Mod19Field, Mod1125899839733759Field};

    use quickcheck::TestResult;

    #[test]
    fn smoky() {
        let elem1: MontgomeryElement<Mod19Field> = 6.into();
        assert_eq!(elem1.into_value(), 1);

        let elem2: MontgomeryElement<Mod19Field> = 16.into();
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

     #[test]
     fn mul_scalar() {
         let elem1: MontgomeryElement<Mod19Field> = 6.into();
         assert_eq!(elem1.clone().into_value(), 1);
         assert_eq!((elem1 * 2).into_value(), 2);
     }

    fn field1_elem<T: Into<MontgomeryElement<Mod1125899839733759Field>>>(v: T) -> MontgomeryElement<Mod1125899839733759Field>
    {
        v.into()
    }

     quickcheck! {
        fn number_div_by_self_equals_one(x: u64) -> TestResult {
            if x % 1125899839733759 == 0 {
                TestResult::discard()
            } else {
                let x_e = field1_elem(x);

                TestResult::from_bool(x_e / x_e == 1.into())
            }
         }

        fn one_div_number_equals_inverse(x: u64) -> TestResult {
            use arith::ModInv;

            if x % 1125899839733759 == 0 {
                 TestResult::discard()
            } else {
                let x_e = field1_elem(x);

                TestResult::from_bool(
                    MontgomeryElement::from(1) / x_e == x_e.into_reduced_value().inv(1125899839733759).into()
                )
            }
        }

        fn field_multiplication_is_commutative(x: u64, y: u64) -> TestResult {
            if x % 1125899839733759 == 0 {
                TestResult::discard()
            } else {
                let x_e = field1_elem(x);
                let y_e = field1_elem(y);

                TestResult::from_bool(
                    x_e * y_e == y_e * x_e
                )
             }
        }

        fn field_multiplication_is_associative(x: u64, y: u64, z: u64) -> TestResult {
             if x % 1125899839733759 == 0 {
                 TestResult::discard()
             } else {
                let x_e = field1_elem(x);
                let y_e = field1_elem(y);
                let z_e = field1_elem(z);

                TestResult::from_bool(
                    (x_e + y_e) * z_e == y_e * z_e + x_e * z_e
                )
             }
         }
     }
}