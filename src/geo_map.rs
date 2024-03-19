use std::collections::HashMap;

use crate::geo_algebra::GA;

pub struct GAMap {}
impl GAMap {
    pub fn compose(&mut self, other: &Self) {
        // TODO
    }

    pub fn map(&self, index: usize) -> usize {
        // TODO
        index
    }
}

pub struct MappedGA<'a, T: GA> {
    internal: &'a T,
    map: GAMap,
}
impl<'a, T: GA> MappedGA<'a, T> {
    pub fn new(ga: &'a T, map: GAMap) -> Self {
        MappedGA { internal: ga, map }
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
