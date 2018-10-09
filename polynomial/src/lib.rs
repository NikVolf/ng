//! Polynomials on finite fields library

#![warn(missing_docs)]

extern crate ng_field as field;

use std::ops::{Mul, Add};

/// Reprents polynomial on the finite field F with coefficient type T
#[derive(Clone, PartialEq, Debug)]
pub struct Polynomial<T: field::FieldValue> {
    coefs: Vec<T>,
}

impl<T: field::FieldValue> Polynomial<T> {
    /// New polynomial with the set of coefficients
    pub fn new<I: Into<T>>(coefs: Vec<I>) -> Self {
        Polynomial {
            coefs: coefs.into_iter().map(Into::into).collect(),
        }
    }

    /// "One" polynomial (multiplicative identity for the ring)
    pub fn one() -> Self {
        Self::new(vec![T::one()])
    }

    /// Zero polynomial (additive identity for the ring)
    pub fn zero() -> Self {
        Self::new(Vec::<T>::new())
    }

    /// Lagrange interpolation
    pub fn interpolate<I: Into<T>+Copy>(points: Vec<(I, I)>) -> Self
    {

        let mut additive_members = Vec::new();

        for i in 0..points.len() {
            let mut poly = Self::one();
            for j in 0..points.len() {
                if i == j { continue; }

                // (x - x[i]) member
                poly = poly * Self::new(vec![-points[j].0.into(), T::one()])
            }

            let val = poly.clone().eval(points[i].0.into());

            poly = poly * (T::one() / val) * points[i].1.into();

            additive_members.push(poly);
        }

        additive_members.into_iter().fold(Self::zero(), |acc, p| acc + p)
    }

    /// Evaluate polynomial on t
    pub fn eval<I: Into<T>+Copy>(self, t: I) -> T
    {
        let mut result = T::zero();
        let mut power = T::one();

        for c in self.coefs.into_iter() {
            result = result + c * power;
            power = power * t.into();
        }

        result
    }
}

impl<T: field::FieldValue> Mul for Polynomial<T>
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

impl<T: field::FieldValue> Mul<T> for Polynomial<T>
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        let mut coefs = self.coefs;

        for coef in coefs.iter_mut() { *coef = *coef * other }

        Self::new(coefs)
    }
}

impl<T: field::FieldValue> Add for Polynomial<T>
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

    type TestPolynomial = Polynomial<field::FieldElement<Mod1125899839733759Field, u64>>;

    #[test]
    fn mul() {
        // x + 5
        let p1: TestPolynomial = Polynomial::new(vec![5, 1]);

        // x + 1
        let p2: TestPolynomial = Polynomial::new(vec![1, 1]);

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
        let p1: TestPolynomial = Polynomial::new(vec![5, 1]);

        // x + 1
        let p2: TestPolynomial = Polynomial::new(vec![1, 1]);

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

    #[test]
    fn interpolation() {
        let p: TestPolynomial = Polynomial::interpolate(vec![(5, 2)]);
        assert_eq!(p.eval(5), 2.into());

        let p: TestPolynomial = Polynomial::interpolate(vec![
            (13, 5), (7, 2)
        ]);
        assert_eq!(p.clone().eval(13), 5.into());
        assert_eq!(p.eval(7), 2.into());
    }
}