use std::collections::HashMap;

use super::deep_map::DeepMap;

#[derive(Debug, Clone)]
pub struct ShallowMap {
    map: HashMap<usize, usize>,
}
impl ShallowMap {
    pub fn compose(&mut self, other: &Self) {
        todo!()
    }

    pub fn map(&self, index: usize) -> usize {
        todo!()
    }

    pub fn to_deep(&self) -> DeepMap {
        todo!()
    }
}