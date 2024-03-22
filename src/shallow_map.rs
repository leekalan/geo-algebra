use std::{collections::HashMap, task::Context};

use crate::{
    compose::{Compose, ComposeRef, ComposeSelf}, deep_map::DeepMap, geo_algebra::GA, inject::Inject
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
impl ComposeSelf<ShallowMap> for ShallowMap {
    fn compose_self(&mut self, contents: ShallowMap) {
        let mut contents = contents;
        for value in self.map.values_mut() {
            if let Some(value_o) = contents.map.remove(value) {
                *value = value_o;
            }
        }
        self.map.extend(contents.map);
    }
}
impl<G: GA> ComposeRef<G, ShallowMappedGA<'_, G>> for ShallowMap {
    fn compose_ref(self, contents: &G) -> ShallowMappedGA<G> {
        ShallowMappedGA {
            internal: contents,
            map: self,
        }
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
}
impl<T: GA> GA for ShallowMappedGA<'_, T> {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let arr: Box<[usize]> = index.iter().map(|x| self.map.map(*x)).collect();
        self.internal.get(&arr)
    }
}