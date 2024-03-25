use std::borrow::Borrow;

use super::GA;

/// The "sum" of two  geometric algebras
pub trait Sum<T>: GA {
    type Result: GA;

    /// The "sum" of two  geometric algebras
    fn sum(&self, other: impl Borrow<T>) -> Self::Result;
}
