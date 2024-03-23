use std::{borrow::Borrow, collections::HashMap};

use wrappr::{
    composer::Composer,
    wrapper::{Wrapper, WrapperRef},
};

use crate::{deep_map::DeepMap, geo_algebra::GA};

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
impl Composer<ShallowMap> for ShallowMap {
    fn compose(&mut self, contents: ShallowMap) -> &mut Self {
        let mut contents = contents;
        for value in self.map.values_mut() {
            if let Some(value_o) = contents.map.remove(value) {
                *value = value_o;
            }
        }
        self.map.extend(contents.map);
        self
    }
}
impl<'a, G: GA> WrapperRef<'a, G, ShallowMappedGA<'a, G>> for ShallowMap {
    fn wrap_ref(self, contents: &'a impl Borrow<G>) -> ShallowMappedGA<'a, G> {
        ShallowMappedGA {
            internal: contents.borrow(),
            map: self,
        }
    }
}
impl<'a, T: GA> Wrapper<ShallowMappedGA<'a, T>, ShallowMappedGA<'a, T>> for ShallowMap {
    fn wrap(mut self, contents: ShallowMappedGA<T>) -> ShallowMappedGA<T> {
        self.compose(contents.map);
        self.wrap_ref(contents.internal)
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
