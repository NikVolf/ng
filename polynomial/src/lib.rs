//! Polynomials on finite fields library

#![warn(missing_docs)]

extern crate ng_field as field;
#[cfg(test)] #[macro_use] extern crate quickcheck;

use std::ops::{Mul, Add};

/// Represents polynomial on the finite field of members type T
///
/// # Examples
///
/// ```
/// extern crate ng_field;
/// extern crate ng_polynomial;
///
/// use ng_polynomial::Polynomial;
/// use ng_field::{Field, FieldElement};
///
/// #[derive(Clone, Copy, PartialEq, Debug)]
/// pub struct Mod1125899839733759Field;
///
/// impl Field for Mod1125899839733759Field {
///     type Value = u64;
///
///     const MODULUS: u64 = 1125899839733759;
///     const R: u64 = 1099511644160;
///     const R_INVERSE: u64 = 343597359104;
/// }
///
/// let polynomial = Polynomial::<FieldElement<Mod1125899839733759Field>>::new(vec![1]);
/// assert_eq!(polynomial, Polynomial::<_>::one())
///
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Polynomial<T: field::FieldValue> {
    coefs: Vec<T>,
}

impl<T: field::FieldValue> Polynomial<T> {
    /// New polynomial with the set of coefficients
    pub fn new<I: IntoIterator<Item=TT>, TT: Into<T>>(coefs: I) -> Self {
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
    pub fn interpolate<I: Into<T>+Copy>(points: &[(I, I)]) -> Self
    {
        let mut additive_members = Vec::new();

        for i in 0..points.len() {
            let mut poly = Self::one();
            for j in 0..points.len() {
                if i == j { continue; }

                // (x - x[i]) member
                poly = poly * Self::new(vec![-points[j].0.into(), T::one()])
            }

            let val = poly.eval(points[i].0.into());

            poly = poly * (T::one() / val) * points[i].1.into();

            additive_members.push(poly);
        }

        additive_members.into_iter().fold(Self::zero(), |acc, p| acc + p)
    }

    /// Evaluate polynomial on t
    pub fn eval<I: Into<T>+Copy>(&self, t: I) -> T
    {
        let mut result = T::zero();
        let mut power = T::one();

        for c in self.coefs.iter() {
            result = result + *c * power;
            power = power * t.into();
        }

        result
    }

    /// Turn polynomial into the array of degrees
    pub fn into_coefs(self) -> Vec<T> {
        self.coefs
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

    use quickcheck::TestResult;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Mod1125899839733759Field;

    impl field::Field for Mod1125899839733759Field {
        type Value = u64;

        const MODULUS: u64 = 1125899839733759;
        const R: u64 = 1099511644160;
        const R_INVERSE: u64 = 343597359104;
    }

    type TestPolynomial = Polynomial<field::FieldElement<Mod1125899839733759Field>>;

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
            pn.eval(1),
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
            pn.eval(1),
            8.into(),
        );

        // f(x) = 2x + 6, f(101) = 208
        assert_eq!(
            pn.eval(101),
            208.into(),
        );
    }

    #[test]
    fn interpolation() {
        let p: TestPolynomial = Polynomial::interpolate(&[(5, 2)]);
        assert_eq!(p.eval(5), 2.into());

        let p: TestPolynomial = Polynomial::interpolate(&[
            (13, 5), (7, 2)
        ]);
        assert_eq!(p.eval(13), 5.into());
        assert_eq!(p.eval(7), 2.into());
    }

    quickcheck! {
        fn interpolation_of_tuple(x1: u64, y1: u64, x2: u64, y2: u64) -> TestResult {
            if y1 == y2 {
                return TestResult::discard();
            }

            if x1 == x2 && y1 != y2 {
                return TestResult::discard();
            }

            let p: TestPolynomial = TestPolynomial::interpolate(&[
                (x1, y1), (x2, y2)
            ]);

            TestResult::from_bool(
                p.eval(x1) == y1.into() &&
                p.eval(x2) == y2.into()
            )
        }

        fn multiplicative_identity(x1: u64, y1: u64, x2: u64, y2: u64) -> TestResult {
            if y1 == y2 {
                return TestResult::discard();
            }

            if x1 == x2 && y1 != y2 {
                return TestResult::discard();
            }

            let mut p: TestPolynomial = TestPolynomial::interpolate(&[
                (x1, y1), (x2, y2)
            ]);

            p = p * TestPolynomial::one();

            TestResult::from_bool(
                p.eval(x1) == y1.into() &&
                p.eval(x2) == y2.into()
            )
        }
    }
}