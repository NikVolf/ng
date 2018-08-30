//! Abstract arithmetic

use std::ops::{Add, Mul, Rem};

pub trait Value: Sized + Add<Output=Self> + Mul<Output=Self> + Rem<Output=Self> + Copy + Clone {
    fn one() -> Self;
    fn zero() -> Self;
}