extern crate pcurve_field as field;
extern crate pcurve_uint as uint;
extern crate curve;
extern crate bigint;

use uint::U256;
use field::{FieldElement, FieldValue};
use curve::{Curve, AffinePoint};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Secp256k1Field;

impl field::Field for Secp256k1Field {
    type Value = U256;

    // 115792089237316195423570985008687907853269984665640564039457584007908834671663
    const MODULUS: U256 = U256(bigint::U256(
        [
            0xFFFFFFFEFFFFFC2F,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
        ]
    ));

    // don't use it for montgomery form
    const R: U256 = U256(bigint::U256([0, 0, 0, 0]));
    const R_INVERSE: U256 = U256(bigint::U256([0, 0, 0, 0]));
}

impl Secp256k1Field {
    pub fn from_str(v: &'static str) -> FieldElement<Self, U256> {
        U256::from(v).into()
    }

    pub fn from_u64(x: u64) -> FieldElement<Self, U256> {
        U256(x.into()).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Secp256k1Curve;

impl Curve for Secp256k1Curve {
    type Value = FieldElement<Secp256k1Field, U256>;

    //
    // 55066263022277343669578718895168534326250603453777594175500187360389116729240,
    // 32670510020758816978083085130507043184471273380659243275938904335757337482424
    //
    fn generator() -> AffinePoint<Self> {
        (
            U256::from_raw([
                0x59F2815B16F81798,
                0x029BFCDB2DCE28D9,
                0x55A06295CE870B07,
                0x79BE667EF9DCBBAC,
            ]),
            U256::from_raw([
                0x9C47D08FFB10D4B8,
                0xFD17B448A6855419,
                0x5DA4FBFC0E1108A8,
                0x483ADA7726A3C465,
            ])
        ).into()
    }

    fn a() -> Self::Value { Self::Value::zero() }
}

#[cfg(test)]
mod tests {

    use super::{Secp256k1Curve, Secp256k1Field};
    use curve::Curve;
    use uint::U256;

    #[test]
    fn curve_add() {

        let p1 = Secp256k1Curve::generator();

        assert_eq!(
            p1,
            (
                Secp256k1Field::from_str("55066263022277343669578718895168534326250603453777594175500187360389116729240"),
                Secp256k1Field::from_str("32670510020758816978083085130507043184471273380659243275938904335757337482424"),
            ).into()
        );

        let p2 = p1.clone();

        assert_eq!(p1.clone() + p2,
            (
                Secp256k1Field::from_str("89565891926547004231252920425935692360644145829622209833684329913297188986597"),
                Secp256k1Field::from_str("12158399299693830322967808612713398636155367887041628176798871954788371653930"),
            ).into()
        );

        let scalar = U256::from("344663216245025");
        assert_eq!(p1 * scalar,
            (
                Secp256k1Field::from_str("105473174440024184228310564028979217580645191183743091203649835187059270886300"),
                Secp256k1Field::from_str("99555671613707051310000045784691741812112923881629020199414035212856909443470"),
            ).into()
        )
    }
}