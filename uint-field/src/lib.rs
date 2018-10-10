extern crate bigint;
extern crate ng_field as field;
#[cfg(test)] #[macro_use] extern crate quickcheck;

use field::{MulReduce, ModMul, ModAdd, ModNeg, ModInv, Scalar};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct U256(pub bigint::U256);

impl U256 {
    pub fn from_raw(v: [u64; 4]) -> Self {
        U256(bigint::U256(v))
    }

    pub fn from_hex(v: &'static str) -> Self {
        use std::str::FromStr;
        U256(bigint::U256::from_str(v).unwrap())
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

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct BtcField;

    use quickcheck::{TestResult, Arbitrary, Gen};
    use {bigint, field};
    use field::FieldElement;
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
        fn from_str(v: &'static str) -> FieldElement<Self> {
            U256::from(v).into()
        }

        fn from_u64(x: u64) -> FieldElement<Self> {
            U256(x.into()).into()
        }
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

    quickcheck! {
        fn number_div_by_self_equals_one(x: U256) -> TestResult {
            use field::Field;

            if x % BtcField::MODULUS == U256(0.into()) {
                TestResult::discard()
            } else {
                let x_e: FieldElement<BtcField> = x.into();

                TestResult::from_bool(x_e / x_e == BtcField::from_u64(1))
            }
        }

        fn one_div_number_equals_inverse(x: U256) -> TestResult {
            use field::{Field, ModInv};

            if x % BtcField::MODULUS == U256(0.into()) {
                TestResult::discard()
            } else {
                let x_e: FieldElement<BtcField> = x.into();

                TestResult::from_bool(
                    BtcField::from_u64(1) / x_e ==
                    x_e.into_value().inv(BtcField::MODULUS).into()
                )
            }
        }
    }
}