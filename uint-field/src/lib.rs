extern crate bigint;
extern crate pcurve_field as field;
#[cfg(test)] #[macro_use] extern crate quickcheck;

use field::{MulReduce, ModMul, ModAdd, ModNeg, ModInv, Scalar};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct U256(bigint::U256);

impl U256 {
    pub fn from_raw(v: [u64; 4]) -> Self {
        U256(bigint::U256(v))
    }
}

impl ModMul for U256 {
    fn mul(self, other: Self, module: Self) -> Self {
        U256(
            (bigint::U512::from(self.0) * bigint::U512::from(other.0) % bigint::U512::from(module.0)).into()
        )
    }
}

impl ModAdd for U256 {
    fn add(self, other: Self, module: Self) -> Self {
        U256(
            ((bigint::U512::from(self.0) + bigint::U512::from(other.0)) % bigint::U512::from(module.0)).into()
        )
    }
}

impl MulReduce for U256 {
    fn mul_reduce(self, other: Self, module: Self, r_inverse: Self) -> Self {
        self.mul(other, module).mul(r_inverse, module)
    }
}

impl ModNeg for U256 {
    fn neg(self, module: Self) -> Self {
        U256(module.0 - (self.0 % module.0))
    }
}

impl ModInv for U256 {
    fn inv(self, module: Self) -> Self {
        U256(self.0.mod_inverse(module.0))
    }
}

impl ModMul<u32> for U256 {
    fn mul(self, scalar: u32, module: Self) -> Self {
        // todo: overflow?
        self.mul(U256(scalar.into()), module)
    }
}

impl ::std::ops::Rem for U256 {
    type Output = U256;
    fn rem(self, other: Self) -> Self {
        U256(self.0 % other.0)
    }
}

impl From<&'static str> for U256 {
    fn from(s: &'static str) -> Self {
        U256(bigint::U256::from_dec_str(s).unwrap())
    }
}

impl Scalar for U256 {
    fn one() -> Self {
        U256(1u64.into())
    }

    fn zero() -> Self {
        U256(0u64.into())
    }

    fn max_bits() -> usize {
        256
    }

    fn bit(&self, position: usize) -> bool {
        self.0.bit(position)
    }
}

#[cfg(test)]
mod tests {

    extern crate curve;

    use self::curve::{Curve, AffinePoint};

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct BtcField;

    use quickcheck::{TestResult, Arbitrary, Gen};
    use {bigint, field};
    use field::{FieldElement, FieldValue};
    use super::U256;

    impl Arbitrary for U256 {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            U256::from_raw([u64::arbitrary(g), u64::arbitrary(g), u64::arbitrary(g), u64::arbitrary(g)])
        }
    }

    impl field::Field for BtcField {
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

    impl BtcField {
        fn from_str(v: &'static str) -> FieldElement<Self, U256> {
            U256::from(v).into()
        }

        fn from_u64(x: u64) -> FieldElement<Self, U256> {
            U256(x.into()).into()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct BtcCurve;

    impl Curve for BtcCurve {
        type Value = FieldElement<BtcField, U256>;

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

    #[test]
    fn field1() {
        let p1 = BtcField::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007908834671663"
        );

        assert_eq!(p1, BtcField::from_str("0"))
    }

    #[test]
    fn field2() {
        let p1 = BtcField::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007908834671662"
        );

        assert_eq!(p1 + p1, p1 - BtcField::from_str("1"))
    }

    #[test]
    fn field3() {
        // P - 1
        let p1 = BtcField::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007908834671662"
        );

        // (P - 1) * 10 = -10
        assert_eq!(
            p1 * BtcField::from_str("10"),
            -BtcField::from_str("10"),
        );
    }

    #[test]
    fn field4() {
        let p1 = -BtcField::from_str("1");

        assert_eq!(
            p1*p1,
            BtcField::from_str("1")
        )
    }

    #[test]
    fn field5() {
        let p1 = BtcField::from_str("44828909320452647301050893743220441820626641267700871452737776290545709136354");
        let p2 = BtcField::from_str("32670510020758816978083085130507043184471273380659243275938904335757337482424");

        assert_eq!(
            p1 - p2,
            BtcField::from_str("12158399299693830322967808612713398636155367887041628176798871954788371653930")
        )
    }

    #[test]
    fn curve_add() {

        let p1 = BtcCurve::generator();

        assert_eq!(
            p1,
            (
                BtcField::from_str("55066263022277343669578718895168534326250603453777594175500187360389116729240"),
                BtcField::from_str("32670510020758816978083085130507043184471273380659243275938904335757337482424"),
            ).into()
        );

        let p2 = p1.clone();

        assert_eq!(p1 + p2,
            (
                BtcField::from_str("89565891926547004231252920425935692360644145829622209833684329913297188986597"),
                BtcField::from_str("12158399299693830322967808612713398636155367887041628176798871954788371653930"),
            ).into()
        );
    }

    quickcheck! {
        fn number_div_by_self_equals_one(x: U256) -> TestResult {
            use field::Field;

            if x % BtcField::MODULUS == U256(0.into()) {
                TestResult::discard()
            } else {
                let x_e: FieldElement<BtcField, _> = x.into();

                TestResult::from_bool(x_e / x_e == BtcField::from_u64(1))
            }
        }

        fn one_div_number_equals_inverse(x: U256) -> TestResult {
            use field::{Field, ModInv};

            if x % BtcField::MODULUS == U256(0.into()) {
                TestResult::discard()
            } else {
                let x_e: FieldElement<BtcField, _> = x.into();

                TestResult::from_bool(
                    BtcField::from_u64(1) / x_e ==
                    x_e.into_value().inv(BtcField::MODULUS).into()
                )
            }
        }
    }
}