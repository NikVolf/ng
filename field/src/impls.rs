use arith::*;

impl ModuleMul for u64 {
    fn mul(self, other: Self, module: Self) -> Self {
        ((self as u128) * (other as u128) % (module as u128)) as u64
    }
}

impl ModuleAdd for u64 {
    fn add(self, other: Self, module: Self) -> Self {
        let (res, overflow) = self.overflowing_add(other);
        if overflow {
            u64::max_value() % module + 1
        } else {
            res % module
        }
    }
}

impl ModuleNeg for u64 {
    fn neg(self, module: Self) -> Self {
        module - (self % module)
    }
}
