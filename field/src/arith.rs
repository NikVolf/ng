//! Abstract arithmetic

/// Modular multiplication.
pub trait ModuleMul {
    /// Multiplication of self by other of the same type
    fn mul(self, other: Self, module: Self) -> Self;
}

/// Modular addition
pub trait ModuleAdd {
    /// Addition of self with other of the same type
    fn add(self, other: Self, module: Self) -> Self;
}

/// Modular negation
pub trait ModuleNeg {
    /// Negation of self by the modulus
    fn neg(self, module: Self) -> Self;
}

/// Modular multiplicative inverse
pub trait ModuleInv {
    /// Calculate modular multiplicative inverse
    fn inv(self, module: Self) -> Self;
}

/// Modular multiplication with reduction
pub trait MulReduce {
    /// Modular multiplication followed by reduction
    fn mul_reduce(self, other: Self, module: Self, r_inverse: Self) -> Self;
}

/// Scalar interface
pub trait Value: Sized + Clone + Copy + ModuleAdd + ModuleMul + ModuleNeg + ModuleInv + MulReduce {
    /// Multiplicative identity
    fn one() -> Self;
    /// Addition identity
    fn zero() -> Self;
}