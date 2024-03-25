pub mod static_vector;
pub mod dynamic_vector;

use crate::geo_algebra::{dot_product::DotProduct, sum::Sum, GA};

pub trait Vector: GA {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Option<f32>;
}
