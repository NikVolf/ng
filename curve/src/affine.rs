
use field::{FieldValue, MulScalar};
use super::Curve;

#[derive(Clone, PartialEq)]
pub struct Point<C: Curve> {
    x: C::Value,
    y: C::Value,
}

impl<C: Curve> Point<C> {
    pub fn infinity() -> Self {
        Point {
            x: C::Value::zero(),
            y: C::Value::zero(),
        }
    }

    pub fn is_infinity(&self) -> bool {
        self.x == C::Value::zero() && self.y == C::Value::zero()
    }

    pub fn new(x: C::Value, y: C::Value) -> Self {
        Point { x: x, y: y }
    }

    pub fn x(&self) -> C::Value {
        self.x
    }

    pub fn y(&self) -> C::Value {
        self.y
    }
}

impl<C: Curve> ::std::ops::Add for Point<C>{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.is_infinity() && other.is_infinity() { return Self::infinity(); }
        if self.is_infinity() { return other; }
        if other.is_infinity() { return self; }

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

            let l = (x1.squared().mul_scalar(3) + C::a()) / y1.mul_scalar(2);
            let x = l.squared() - x1.mul_scalar(2);

            return Point {
                x: x,
                y: l * (x1 - x) - y1,
            };
        }

        let l = (y2 - y1) / (x2 - x1);
        let x3 = l.squared() - x1 - x2;
        let y3 = l * (x1 - x3) - y1;

        Point {
            x: x3,
            y: y3,
        }
    }
}

#[cfg(test)]
mod tests {

    use field;
    use Curve;
    use super::Point;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct U64Field;

    impl field::Field for U64Field {
        type Value = u64;

        const MODULUS: u64 = 1125899839733759;
        const R: u64 = 1099511644160;
        const R_INVERSE: u64 = 343597359104;
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct U64Curve;

    /// y^2 = x^3 + 7x + 14 over Fp mod 1125899839733759
    impl Curve for U64Curve {
        type Value = field::FieldElement<U64Field, u64>;

        fn generator() -> Point<Self> {
            Point::new(
                2.into(),
                6.into(),
            )
        }

        fn a() -> Self::Value {
            7.into()
        }
    }

    #[test]
    fn doubling() {
        let p = U64Curve::generator();
        let dp = p.clone() + p;

        // 570768668753918, 222182780873386
        assert_eq!(dp.x(), 570768668753918.into());
        assert_eq!(dp.y(), 222182780873386.into());
    }

    #[test]
    fn add() {
        let p = U64Curve::generator() + U64Curve::generator();
        let np = p.clone() + U64Curve::generator();

        assert_eq!(np.x(), 537613624567015.into());
        assert_eq!(np.y(), 945163207984607.into());
    }
}