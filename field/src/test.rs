
use field;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mod19Field;

impl field::Field for Mod19Field {
    type Value = u64;
    const MODULUS: Self::Value = 19;
    const R: Self::Value = 16;
    const R_INVERSE: Self::Value = 6;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mod1125899839733759Field;

impl field::Field for Mod1125899839733759Field {
    type Value = u64;

    const MODULUS: u64 = 1125899839733759;
    const R: u64 = 1099511644160;
    const R_INVERSE: u64 = 343597359104;
}