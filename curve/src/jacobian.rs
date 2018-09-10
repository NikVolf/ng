use field::{FieldValue, MulScalar, Scalar};
use {Curve, AffinePoint};

/// Point on the curve C in jacobian representation
#[derive(Clone, Debug, PartialEq)]
pub struct Point<C: Curve> {
    x: C::Value,
    y: C::Value,
    z: C::Value,
}

impl<C: Curve> From<AffinePoint<C>> for Point<C> {
    fn from(p: AffinePoint<C>) -> Self {
        let (x, y) = p.into_parts();

        Point {
            x: x,
            y: y,
            z: C::Value::one(),
        }
    }
}

impl<I, C: Curve> From<(I, I, I)> for Point<C>
    where I: Into<C::Value>
{
    fn from(p: (I, I, I)) -> Self {
        Point {
            x: p.0.into(),
            y: p.1.into(),
            z: p.2.into(),
        }
    }
}

impl<C: Curve> Point<C> {
    /// Deconstruct point into (X, Y, Z) parts
    pub fn into_parts(self) -> (C::Value, C::Value, C::Value) {
        (self.x, self.y, self.z)
    }

    /// New point at infinity
    pub fn infinity() -> Self {
        Point {
            x: C::Value::zero(),
            y: C::Value::one(),
            z: C::Value::zero(),
        }
    }

    /// Test if point is at infinity
    pub fn is_infinity(&self) -> bool {
        self.z == C::Value::zero()
    }
}

impl<I: Scalar, C: Curve> ::std::ops::Mul<I> for Point<C>
{
    type Output = Self;

    fn mul(self, other: I) -> Self {
        let mut r0 = Self::infinity();
        let mut r1 = self;
        for i in 0..I::max_bits() {
            let b = Scalar::bit(&other, i);
            if b { r0 = r0 + r1.clone() }
            r1 = r1.clone() + r1;
        }
        r0
    }
}

impl<I: Into<Point<C>>, C: Curve> ::std::ops::Add<I> for Point<C> {
    type Output = Self;
    fn add(self, other: I) -> Self {
        let other: Point<C> = other.into();
        if self.is_infinity() && other.is_infinity() { return Self::infinity(); }
        if self.is_infinity() {
            return other;
        }
        if other.is_infinity() { return self; }

        let (x1, y1, z1) = self.into_parts();
        let (x2, y2, z2) = other.into_parts();

        // U1 = X1*Z2^2
        let u1 = x1 * z2.squared();
        // U2 = X2*Z1^2
        let u2 = x2 * z1.squared();

        // S1 = Y1*Z2^3
        let s1 = y1 * (z2.squared() * z2);
        // S2 = Y2*Z1^3
        let s2 = y2 * (z1.squared() * z1);

        if u1 == u2 {
            if s1 != s2 { return Self::infinity(); }
            else {
                // S = 4*X*Y^2
                let s = x1.mul_scalar(4) * y1.squared();

                // M = 3*X^2 + a*Z^4
                // Curve over montgomery field elements should have C::a() in regular form!
                let m = x1.squared().mul_scalar(3) + C::a() * (z1.squared().squared());

                // X' = M^2 - 2*S
                let x = m.squared() - s.mul_scalar(2);

                // Y' = M*(S - X') - 8*Y^4
                let y = m * (s - x) - y1.squared().squared().mul_scalar(8);

                // Z' = 2*Y*Z
                let z = (y1 * z1).mul_scalar(2);

                return (x, y, z).into()
            }
        }

        // H = U2 - U1
        let h = u2 - u1;
        // R = S2 - S1
        let r = s2 - s1;

        let h2 = h.squared();
        let h3 = h2 * h;

        // X3 = R^2 - H^3 - 2*U1*H^2
        let x3 = r.squared() - h3 - u1*h2.mul_scalar(2);

        // Y3 = R*(U1*H^2 - X3) - S1*H^3
        let y3 = r * (u1 * h2 - x3) - s1 * h3;

        // Z3 = H*Z1*Z2
        let z3 = h * z1 * z2;

        (x3, y3, z3).into()
    }
}

#[cfg(test)]
mod tests {

    use test::{U64Curve, U64MontgomeryCurve};
    use {JacobianPoint, AffinePoint, Curve};

    #[test]
    fn double() {
        let jp: JacobianPoint<U64Curve> = U64Curve::generator().into();
        let dp = AffinePoint::from(jp.clone() + jp);

        // 570768668753918, 222182780873386
        assert_eq!(dp, (570768668753918, 222182780873386).into());
    }

    #[test]
    fn add() {
        let jp: JacobianPoint<U64Curve> = U64Curve::generator().into();
        let djp = jp.clone() + jp.clone();
        let np = AffinePoint::from(djp + jp);

        assert_eq!(np, (537613624567015, 945163207984607).into());
    }

    #[test]
    fn mul() {
        let jp: JacobianPoint<U64Curve> = U64Curve::generator().into();
        let dp = AffinePoint::from(jp.clone() * 2);;
        assert_eq!(dp, (570768668753918, 222182780873386).into());
        let bp = AffinePoint::from(jp * 570768668753918);
        assert_eq!(bp, (210159848059198, 473433224346301).into());
    }

    #[test]
    fn mul_mont() {
        let jp: JacobianPoint<U64MontgomeryCurve> = U64MontgomeryCurve::generator().into();
        let dp = AffinePoint::from(jp.clone() * 2);;
        assert_eq!(dp, (570768668753918, 222182780873386).into());
        let bp = AffinePoint::from(jp * 570768668753918);
        assert_eq!(bp, (210159848059198, 473433224346301).into());
    }
}