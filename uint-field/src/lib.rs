extern crate bigint;
extern crate pcurve_field as field;

use field::{MulReduce, ModuleMul, ModuleAdd, ModuleNeg, ModuleInv, ModuleMulScalar, Scalar};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct U256(bigint::U256);

impl ModuleMul for U256 {
    fn mul(self, other: Self, module: Self) -> Self {
        U256(
            (bigint::U512::from(self.0) * bigint::U512::from(other.0) % bigint::U512::from(module.0)).into()
        )
    }
}

impl ModuleAdd for U256 {
    fn add(self, other: Self, module: Self) -> Self {
        let (res, overflow) = self.0.overflowing_add(other.0);
        if overflow {
            U256(bigint::U256::max_value() % module.0 + 1.into())
        } else {
            U256(res % module.0)
        }
    }
}

impl MulReduce for U256 {
    fn mul_reduce(self, other: Self, module: Self, r_inverse: Self) -> Self {
        self.mul(other, module).mul(r_inverse, module)
    }
}

impl ModuleNeg for U256 {
    fn neg(self, module: Self) -> Self {
        U256(module.0 - (self.0 % module.0))
    }
}

impl ModuleInv for U256 {
    fn inv(self, module: Self) -> Self {
        U256(self.0.mod_inverse(module.0))
    }
}

impl ModuleMulScalar for U256 {
    fn mul_scalar(self, scalar: u32, module: Self) -> Self {
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

    use {bigint, field};
    use field::{FieldElement, FieldValue};
    use super::U256;

    impl field::Field for BtcField {
        type Value = U256;

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
    }

    #[test]
    fn field1() {
        let p1 = BtcField::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007908834671663"
        );

        assert_eq!(p1, FieldElement::<BtcField, U256>::zero())
    }

    #[test]
    fn field2() {
        let p1 = BtcField::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007908834671662"
        );

        assert_eq!(p1 + p1, p1 - U256::from("2").into())
    }

    #[test]
    fn field3() {
        // P - 1
        let p1 = BtcField::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007908834671662"
        );

        // P - 1 * 10 = - 10
        assert_eq!(
            p1 * BtcField::from_str("10"),
            -BtcField::from_str("10"),
        );
    }
}