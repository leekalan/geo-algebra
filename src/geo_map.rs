pub mod shallow_map;
pub mod deep_map;

use crate::{collect::CollectFrom, geo_algebra::GA};

use deep_map::DeepMap;
use shallow_map::ShallowMap;

#[derive(Debug, Clone)]
pub enum GAMap {
    Shallow(ShallowMap),
    Deep(DeepMap),
}
impl GAMap {
    pub fn compose(&mut self, other: &Self) {
        match (self, other) {
            (GAMap::Shallow(t0), GAMap::Shallow(t1)) => t0.compose(t1),
            (GAMap::Deep(t0), GAMap::Deep(t1)) => t0.compose(t1),
            (GAMap::Shallow(t0), GAMap::Deep(t1)) => t0.to_deep().compose(t1),
            (GAMap::Deep(t0), GAMap::Shallow(t1)) => t0.compose(&t1.to_deep()),
        }
    }
    pub fn compose_stacked(&self, other: &Self) {
        todo!()
    }

    pub fn map(&self, index: usize) -> usize {
        match self {
            GAMap::Shallow(t0) => t0.map(index),
            GAMap::Deep(t0) => t0.map(index),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MappedGA<'a, T: GA> {
    internal: &'a T,
    map: GAMap,
}
impl<'a, T: GA> MappedGA<'a, T> {
    pub fn new(ga: &'a T, map: GAMap) -> Self {
        MappedGA { internal: ga, map }
    }

    pub fn collect(self) -> T where T: CollectFrom {
        T::collect_from(self)
    }
}
impl<T: GA> GA for MappedGA<'_, T> {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let vec: Box<[usize]> = index.iter().map(|x| self.map.map(*x)).collect();
        self.internal.get(&vec)
    }

    fn map<U: GA>(&self, map: GAMap) -> MappedGA<U>
    where
        Self: AsRef<U>,
    {
        MappedGA::new(self.as_ref(), map)
    }
}
impl<T: GA + AsRef<U>, U: GA> AsRef<U> for MappedGA<'_, T> {
    fn as_ref(&self) -> &U {
        self.internal.as_ref()
    }
}
