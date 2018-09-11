//! Abstract field and field element operations implementation

#![warn(missing_docs)]

#[cfg(test)] #[macro_use] extern crate quickcheck;

mod field;
mod element;
mod arith;
mod impls;
mod mont;
#[cfg(test)]
mod test;

pub use arith::{MulReduce, ModMul, ModAdd, ModInv, ModNeg};
pub use arith::Value as Scalar;
pub use field::{Field, FieldValue};
pub use element::FieldElement;
pub use mont::MontgomeryElement;