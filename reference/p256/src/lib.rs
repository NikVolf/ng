extern crate pcurve_field as field;
extern crate pcurve_uint as uint;
extern crate curve;
extern crate bigint;

use uint::U256;
use field::MontgomeryElement;
use curve::{Curve, AffinePoint};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct P256Field;

impl field::Field for P256Field {
    type Value = U256;

    // 115792089210356248762697446949407573530086143415290314195533631308867097853951
    const MODULUS: U256 = U256(bigint::U256(
        [
            0xffffffffffffffff,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
        ]
    ));

    // 115792089237316195423570985008687907853269984665640564039457584007913129639936
    const R: U256 = U256(bigint::U256(
        [
            0x0000000000000001,
            0xFFFFFFFF00000000,
            0xFFFFFFFFFFFFFFFF,
            0x00000000FFFFFFFE,
        ]
    ));

    // 115792089183396302114378112356516095823261736990586219612555396166510339686400
    const R_INVERSE: U256 = U256(bigint::U256(
        [
            0x0000000300000000,
            0x00000001FFFFFFFE,
            0xFFFFFFFD00000002,
            0xFFFFFFFE00000003,
        ]
    ));
}

impl P256Field {
    pub fn from_str(v: &'static str) -> MontgomeryElement<Self, U256> {
        U256::from(v).into()
    }

    pub fn from_u64(x: u64) -> MontgomeryElement<Self, U256> {
        U256(x.into()).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct P256Curve;

impl Curve for P256Curve {
    type Value = MontgomeryElement<P256Field, U256>;

    //
    // 0x6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296,
    // 0x4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5
    //
    fn generator() -> AffinePoint<Self> {
        (
            U256::from_raw([
                0xF4A13945D898C296,
                0x77037D812DEB33A0,
                0xF8BCE6E563A440F2,
                0x6B17D1F2E12C4247,
            ]),
            U256::from_raw([
                0xCBB6406837BF51F5,
                0x2BCE33576B315ECE,
                0x8EE7EB4A7C0F9E16,
                0x4FE342E2FE1A7F9B,
            ])
        ).into()
    }

    // 115792089210356248762697446949407573530086143415290314195533631308867097853948 (-3 mod p)
    fn a() -> Self::Value {
        U256::from_raw([
            0xFFFFFFFFFFFFFFFC,
            0x00000000FFFFFFFF,
            0x0000000000000000,
            0xFFFFFFFF00000001,
        ]).into()
    }
}

#[cfg(test)]
mod tests {

    use super::{P256Curve, P256Field};
    use curve::Curve;

    #[test]
    fn curve_add() {

        let p1 = P256Curve::generator();

        assert_eq!(
            p1,
            (
                P256Field::from_str("48439561293906451759052585252797914202762949526041747995844080717082404635286"),
                P256Field::from_str("36134250956749795798585127919587881956611106672985015071877198253568414405109"),
            ).into()
        );

        let dp = p1.clone() + p1.clone();

        assert_eq!(dp,
            (
                P256Field::from_str("56515219790691171413109057904011688695424810155802929973526481321309856242040"),
                P256Field::from_str("3377031843712258259223711451491452598088675519751548567112458094635497583569"),
            ).into()
        );

        let tp = dp + p1;

        assert_eq!(tp,
            (
                P256Field::from_str("42877656971275811310262564894490210024759287182177196162425349131675946712428"),
                P256Field::from_str("61154801112014214504178281461992570017247172004704277041681093927569603776562"),
            ).into()
        );
    }
}