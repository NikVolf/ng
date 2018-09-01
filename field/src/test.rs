
use field;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mod19Field;

impl field::Field for Mod19Field {
    type Value = u64;
    const MODULUS: Self::Value = 19;
    const R: Self::Value = 16;
    const R_INVERSE: Self::Value = 6;
}