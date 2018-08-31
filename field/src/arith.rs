//! Abstract arithmetic

pub trait ModuleMul {
    fn mul(self, other: Self, module: Self) -> Self;
}

pub trait ModuleAdd {
    fn add(self, other: Self, module: Self) -> Self;
}

pub trait ModuleNeg {
    fn neg(self, module: Self) -> Self;
}

pub trait ModuleInv {
    fn inv(self, module: Self) -> Self;
}

pub trait MulReduce {
    fn mul_reduce(self, other: Self, module: Self, r_inverse: Self) -> Self;
}

pub trait Value: Sized + Clone + Copy + ModuleAdd + ModuleMul + ModuleNeg + ModuleInv + MulReduce {
    fn one() -> Self;
    fn zero() -> Self;
}