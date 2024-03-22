use crate::{
    compose::ComposeRef,
    deep_map::{DeepMap, DeepMappedGA},
    inject::InjectRef,
    shallow_map::{ShallowMap, ShallowMappedGA},
};

pub trait GA: Sized {
    fn get(&self, index: &[usize]) -> Option<f32>;
    fn create_shallow_map(&self, map: ShallowMap) -> ShallowMappedGA<Self> {
        self.inject_ref(map)
    }
    fn create_deep_map(&self, map: DeepMap) -> DeepMappedGA<Self> {
        map.compose_ref(self)
    }
}
