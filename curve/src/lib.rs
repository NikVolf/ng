extern crate pcurve_field as field;

mod affine;

pub use affine::Point as AffinePoint;

pub struct JacobianPoint<C: Curve> {
    x: C::Value,
    y: C::Value,
    z: C::Value,
}

pub trait Curve : Sized {
    type Value: field::FieldValue;

    fn generator() -> affine::Point<Self>;

    fn a() -> Self::Value;
}