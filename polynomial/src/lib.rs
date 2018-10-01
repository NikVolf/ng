extern crate pcurve_field as field;

use std::ops::{Mul, Add, Neg};
use field::FieldElement;

#[derive(Clone, PartialEq, Debug)]
pub struct Polynomial<F: field::Field<Value=T>, T: field::Scalar> {
    coefs: Vec<T>,
    _field: ::std::marker::PhantomData<F>,
}

impl<F: field::Field<Value=T>, T: field::Scalar> Polynomial<F, T> {
    pub fn new(coefs: Vec<T>) -> Self {
        Polynomial {
            coefs: coefs,
            _field: ::std::marker::PhantomData,
        }
    }

    pub fn eval(self, t: T) -> FieldElement<F, T>
        where T: Mul<Output=T>,
            FieldElement<F,T>: Mul<Output=FieldElement<F,T>>
    {
        let mut result: FieldElement<F, T> = T::zero().into();
        let mut power: FieldElement<F, T> = T::one().into();

        for c in self.coefs.into_iter() {
            let c_val: FieldElement<F, T> = c.into();
            result = result + c_val * power;
            power = power * t.into();
        }

        result
    }
}

impl<F: field::Field<Value=T>, T: field::Scalar> Mul for Polynomial<F, T>
    where T: Mul<Output=T>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Vec::new();
        for self_c in self.coefs.into_iter() {
            for other_c in other.coefs.iter() {
                result.push(self_c * other_c.clone());
            }
        }
        Self::new(result)
    }
}

#[cfg(test)]
mod tests {
    use field;
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Mod1125899839733759Field;

    impl field::Field for Mod1125899839733759Field {
        type Value = u64;

        const MODULUS: u64 = 1125899839733759;
        const R: u64 = 1099511644160;
        const R_INVERSE: u64 = 343597359104;
    }

    #[test]
    fn mul() {
        let p1: Polynomial<Mod1125899839733759Field, _> = Polynomial::new(vec![5, 1]);
        let p2: Polynomial<Mod1125899839733759Field, _> = Polynomial::new(vec![1, 1]);

        let pN = p1 * p2;

        assert_eq!(
            pN.clone().eval(1),
            12.into(),
        );

        assert_eq!(
            pN.eval(2),
            21.into(),
        );
    }
}