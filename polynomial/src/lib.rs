extern crate pcurve_field as field;

use std::ops::{Mul, Add};
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
    where T: Mul<Output=T>, T: Add<Output=T>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let order = self.coefs.len().max(other.coefs.len());

        let mut result = Vec::with_capacity(order*2);
        result.resize(order*2, T::zero());

        for i in 0..order {
            for j in 0..order {
                let c_order = i + j;

                result[c_order] = result[c_order] +
                    self.coefs.get(i).unwrap_or(&T::zero()).clone() *
                    other.coefs.get(j).unwrap_or(&T::zero()).clone();
            }
        }

        Self::new(result)
    }
}

impl<F: field::Field<Value=T>, T: field::Scalar> Add for Polynomial<F, T>
    where T: Add<Output=T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let order = self.coefs.len().max(other.coefs.len());

        let mut result = Vec::with_capacity(order);
        result.resize(order, T::zero());

        for i in 0..order {
            result[i] =
                self.coefs.get(i).unwrap_or(&T::zero()).clone() +
                other.coefs.get(i).unwrap_or(&T::zero()).clone();
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
        // x + 5
        let p1: Polynomial<Mod1125899839733759Field, _> = Polynomial::new(vec![5, 1]);

        // x + 1
        let p2: Polynomial<Mod1125899839733759Field, _> = Polynomial::new(vec![1, 1]);

        // x^2 + 6x + 5
        let pn = p1 * p2;

        // f(x) = x^2 + 6x + 5, f(1) = 12
        assert_eq!(
            pn.clone().eval(1),
            12.into(),
        );

        // f(x) = x^2 + 6x + 5, f(2) = 21
        assert_eq!(
            pn.eval(2),
            21.into(),
        );
    }

    #[test]
    fn add() {
        // x + 5
        let p1: Polynomial<Mod1125899839733759Field, _> = Polynomial::new(vec![5, 1]);

        // x + 1
        let p2: Polynomial<Mod1125899839733759Field, _> = Polynomial::new(vec![1, 1]);

        // 2x + 6
        let pn = p1 + p2;

        // f(x) = 2x + 6, f(1) = 8
        assert_eq!(
            pn.clone().eval(1),
            8.into(),
        );

        // f(x) = 2x + 6, f(101) = 208
        assert_eq!(
            pn.clone().eval(101),
            208.into(),
        );
    }
}