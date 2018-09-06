use field;
use {Curve, AffinePoint};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct U64Field;

impl field::Field for U64Field {
    type Value = u64;

    const MODULUS: u64 = 1125899839733759;
    const R: u64 = 1099511644160;
    const R_INVERSE: u64 = 343597359104;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct U64Curve;

/// y^2 = x^3 + 7x + 14 over Fp mod 1125899839733759
impl Curve for U64Curve {
    type Value = field::FieldElement<U64Field, u64>;

    fn generator() -> AffinePoint<Self> {
        (2, 6).into()
    }

    fn a() -> Self::Value {
        7.into()
    }
}