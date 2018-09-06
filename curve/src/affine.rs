
use field::{FieldValue, MulScalar};
use super::Curve;

pub struct Point<C: Curve> {
    x: C::Value,
    y: C::Value,
}

impl<C: Curve> Point<C> {
    fn infinity() -> Self {
        Point {
            x: C::Value::zero(),
            y: C::Value::zero(),
        }
    }
}

impl<C: Curve> ::std::ops::Add for Point<C>{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = other.x;
        let y2 = other.y;
        if x1 == x2 {
            // point doubling
            if y1 != y2 {
                return Self::infinity();
            }

            if y1 == C::Value::zero() {
                return Self::infinity();
            }

            let l = x1.squared().mul_scalar(3) / (y1.mul_scalar(2));
            let x = l.squared() - x1.mul_scalar(2);

            return Point {
                x: x,
                y: l * (x1 - x) - y1,
            };
        }
        Self::infinity()
    }
}
