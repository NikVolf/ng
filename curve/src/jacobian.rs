use field::{FieldValue, MulScalar};
use {Curve, AffinePoint};

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
    fn into_parts(self) -> (C::Value, C::Value, C::Value) {
        (self.x, self.y, self.z)
    }

    pub fn infinity() -> Self {
        Point {
            x: C::Value::zero(),
            y: C::Value::one(),
            z: C::Value::one(),
        }
    }
}

impl<I: Into<Point<C>>, C: Curve> ::std::ops::Add<I> for Point<C> {
    type Output = Self;
    fn add(self, other: I) -> Self {
        let other: Point<C> = other.into();

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
                let m = x1.squared().mul_scalar(3) + C::a() * z1.squared().squared();

                // X' = M^2 - 2*S
                let x = m.squared() - s.mul_scalar(2);

                // Y' = M*(S - X') - 8*Y^4
                let y = m * (s - x) - y1.squared().squared().mul_scalar(8);

                // Z' = 2*Y*Z
                let z = (y * z1).mul_scalar(2);

                return (x, y, z).into()
            }
        }

        Self::infinity()
    }
}