//! Abstract field

/// Abstract field description
pub trait Field {
    /// Type of scalar
    type Value;
    /// Field modulus
    const MODULUS: Self::Value;
    /// Field montgomery coefficient (R)
    const R: Self::Value;
    /// Multiplicative inverse of R modulus MODULUS
    const R_INVERSE: Self::Value;
}