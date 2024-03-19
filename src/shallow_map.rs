use std::collections::HashMap;

use crate::{
    collect::{CollectFromDeep, CollectFromShallow},
    deep_map::{DeepMap, DeepMappedGA},
    geo_algebra::GA,
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

    pub fn compose(&mut self, other: &Self) {
        let mut values = self.map.values_mut();

        let mut extra = Vec::new();
        for (key_o, value_o) in &other.map {
            if let Some(value_m) = values.find(|v| **v == *key_o) {
                *value_m = *value_o;
            } else {
                extra.push((*key_o, *value_o));
            }
        }

        self.map.extend(extra);
    }

    pub fn to_deep(&self) -> DeepMap {
        todo!()
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

    pub fn collect<U>(&self) -> U
    where
        T: CollectFromShallow<U>,
    {
        T::collect_from_shallow(self)
    }
}
impl<T: GA> GA for ShallowMappedGA<'_, T> {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let arr: Box<[usize]> = index.iter().map(|x| self.map.map(*x)).collect();
        self.internal.get(&arr)
    }
}
impl<T: GA> CollectFromShallow<Self> for ShallowMappedGA<'_, T> {
    fn collect_from_shallow(mapped: &ShallowMappedGA<Self>) -> Self {
        let mut map = mapped.map.clone();
        map.compose(&mapped.internal.map);
        Self {
            internal: mapped.internal.internal,
            map,
        }
    }
}
impl<'a, T: GA> CollectFromDeep<DeepMappedGA<'a, T>> for ShallowMappedGA<'a, T> {
    fn collect_from_deep(mapped: &DeepMappedGA<Self>) -> DeepMappedGA<'a, T> {
        let mut map = mapped.map.clone();
        map.compose(&mapped.internal.map.to_deep());
        DeepMappedGA {
            internal: mapped.internal.internal,
            map,
        }
    }
}
