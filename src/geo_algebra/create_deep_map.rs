use crate::deep_map::DeepMap;

use super::GA;

pub trait CreateDeepMap: GA {
    type Result;

    /// Create a shallow map over the geometric algebra.
    fn create_shallow_map(&self, map: DeepMap) -> Self::Result;
}