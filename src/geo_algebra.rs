use crate::{
    deep_map::{DeepMap, DeepMappedGA},
    shallow_map::{ShallowMap, ShallowMappedGA},
};

pub trait GA: Sized {
    fn get(&self, index: &[usize]) -> Option<f32>;
    fn shallow_map(&self, map: ShallowMap) -> ShallowMappedGA<Self> {
        ShallowMappedGA::new(self, map)
    }
    fn deep_map(&self, map: DeepMap) -> DeepMappedGA<Self> {
        DeepMappedGA::new(self, map)
    }
}
