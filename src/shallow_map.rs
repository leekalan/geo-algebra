use std::collections::HashMap;

use crate::{
    collect::Compose, deep_map::DeepMap, geo_algebra::GA
};

#[derive(Debug, Clone)]
pub struct ShallowMap {
    map: HashMap<usize, usize>,
}
impl ShallowMap {
    pub fn new(map: HashMap<usize, usize>) -> Self {
        Self { map }
    }

    pub fn map(&self, index: usize) -> usize {
        match self.map.get(&index) {
            Some(t0) => *t0,
            None => index,
        }
    }

    pub fn to_deep(&self) -> DeepMap {
        todo!()
    }
}
impl Compose<ShallowMap> for ShallowMap {
    fn compose(&mut self, other: Self) {
        let mut other = other;
        for value in self.map.values_mut() {
            if let Some(value_o) = other.map.remove(value) {
                *value = value_o;
            }
        }
        self.map.extend(other.map);
    }
}

#[derive(Debug, Clone)]
pub struct ShallowMappedGA<'a, T: GA> {
    pub internal: &'a T,
    pub map: ShallowMap,
}
impl<'a, T: GA> ShallowMappedGA<'a, T> {
    pub fn new(ga: &'a T, map: ShallowMap) -> Self {
        Self { internal: ga, map }
    }

    pub fn get_internal(self) -> &'a T {
        self.internal
    }

    pub fn compose(&mut self, map: ShallowMap) {
        let mut map = map;
        std::mem::swap(&mut self.map, &mut map);
        self.map.compose(map);
    }
}
impl<T: GA> GA for ShallowMappedGA<'_, T> {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let arr: Box<[usize]> = index.iter().map(|x| self.map.map(*x)).collect();
        self.internal.get(&arr)
    }
}