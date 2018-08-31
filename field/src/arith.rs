//! Abstract arithmetic

pub trait MulReduce {
    fn mul_reduce(self, other: Self, module: Self) -> Self;
}

pub trait AddReduce {
    fn add_reduce(self, other: Self, module: Self) -> Self;
}

impl MulReduce for u64 {
    fn mul_reduce(self, other: Self, module: Self) -> Self {
        ((self as u128) * (other as u128) % (module as u128)) as u64
    }
}

impl AddReduce for u64 {
    fn add_reduce(self, other: Self, module: Self) -> Self {
        let (res, overflow) = self.overflowing_add(other);
        if overflow {
            u64::max_value() % module + 1
        } else {
            res % module
        }
    }
}

pub trait Value: Sized + Clone + Copy + AddReduce + MulReduce {
    fn one() -> Self;
    fn zero() -> Self;
}