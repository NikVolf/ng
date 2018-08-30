//! Abstract field

pub trait Field {
    type Value;
    fn modulus() -> Self::Value;
}