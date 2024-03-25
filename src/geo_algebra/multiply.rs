use std::borrow::Borrow;

use super::GA;

/// The "product" of two geometric algebras
pub trait Mutiply<T>: GA {
    type Result: GA;

    /// The "product" of two geometric algebras
    fn mul(&self, other: impl Borrow<T>) -> Self::Result;
}
