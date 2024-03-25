use crate::shallow_map::ShallowMap;

use super::GA;

pub trait CreateShallowMap<'a>: GA {
    type Result;
    
    /// Create a shallow map over the geometric algebra.
    fn create_shallow_map(&'a self, map: ShallowMap) -> Self::Result;
}