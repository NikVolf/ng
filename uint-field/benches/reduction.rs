#![feature(test)]

extern crate test;
extern crate ng_uint as uint;
extern crate ng_field as field;
extern crate bigint;

use uint::U256;
use field::MontgomeryElement;

#[derive(Copy, Clone, Debug, PartialEq)]
struct P256Field;

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
    pub fn from_str(v: &'static str) -> MontgomeryElement<Self> {
        U256::from(v).into()
    }
}

#[bench]
fn simple(b: &mut test::Bencher) {
    let mut val = MontgomeryElement::from(P256Field::from_str("11"));
    let multiplyer = MontgomeryElement::from(P256Field::from_str("13"));
    b.iter(|| {
        val = val * multiplyer;
    });
}