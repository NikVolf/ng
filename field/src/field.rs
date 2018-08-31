//! Abstract field

pub trait Field {
    type Value;
    const MODULUS: Self::Value;
}