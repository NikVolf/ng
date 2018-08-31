//! Abstract field

pub trait Field {
    type Value;
    const MODULUS: Self::Value;
    const R: Self::Value;
    const R_INVERSE: Self::Value;
}