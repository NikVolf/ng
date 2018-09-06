extern crate pcurve_field as field;

struct AffinePoint<V: field::FieldValue> {
    x: V,
    y: V,
}

struct JacobianPoint<V: field::Value> {
    x: V,
    y: V,
    z: V,
}

trait Curve {
    type Value: field::FieldValue;

    const GENERATOR: AffinePoint<Self::Value>;
}