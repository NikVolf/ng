//! Library for elliptic curve generalization

#![warn(missing_docs)]

extern crate ng_field as field;

mod affine;
mod jacobian;

#[cfg(test)]
mod test;

pub use affine::Point as AffinePoint;
pub use jacobian::Point as JacobianPoint;

/// Generalization of a y^2 = x^2 + ax + b curve
pub trait Curve : Sized + Clone {
    /// Field element type of the curve
    type Value: field::FieldValue;

    /// Generator point of the curve
    fn generator() -> affine::Point<Self>;

    /// a-parameter of the curve
    fn a() -> Self::Value;
}