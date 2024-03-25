use std::borrow::Borrow;

use super::GA;

pub trait DotProduct<T: GA>: GA {
    fn dot_product(&self, other: impl Borrow<T>) -> f32;
}
