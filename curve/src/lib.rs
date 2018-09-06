extern crate pcurve_field as field;

mod affine;
mod jacobian;

pub use affine::Point as AffinePoint;
pub use jacobian::Point as JacobianPoint;

pub trait Curve : Sized {
    type Value: field::FieldValue;

    fn generator() -> affine::Point<Self>;

    fn a() -> Self::Value;
}