use arith::*;

impl ModuleMul for u64 {
    fn mul(self, other: Self, module: Self) -> Self {
        ((self as u128) * (other as u128) % (module as u128)) as u64
    }
}

impl MulReduce for u64 {
    fn mul_reduce(self, other: Self, module: Self, r_inverse: Self) -> Self {
        self.mul(other, module).mul(r_inverse, module)
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

impl ModuleInv for u64 {
    fn inv(self, module: Self) -> Self {
		let mut mn = (module, self);
		let mut xy = (0, 1);

		while mn.1 != 0 {
			let sb = (mn.0 / mn.1).mul(xy.1, module);
			if sb > xy.0 {
				xy = (xy.1, module - ((sb - xy.0) % module))
			} else {
				xy = (xy.1, xy.0 - sb)
			}
			mn = (mn.1, mn.0 % mn.1);
		}

		xy.0
    }
}

impl Value for u64 {
    fn one() -> u64 {
        1
    }
    fn zero() -> u64 {
        0
    }
}