use crate::{
    compose::{Compose, ComposeRef, ComposeSelf},
    geo_algebra::GA,
};

#[derive(Debug, Clone)]
pub struct DeepMap {}
impl DeepMap {
    pub fn map(&self, index: &[usize]) -> Box<[usize]> {
        todo!()
    }
}
impl ComposeSelf<DeepMap> for DeepMap {
    fn compose_self(&mut self, contents: DeepMap) {
        todo!()
    }
}
impl<'a, G: GA> ComposeRef<'a, G, DeepMappedGA<'a, G>> for DeepMap {
    fn compose_ref(self, contents: &G) -> DeepMappedGA<G> {
        DeepMappedGA {
            internal: contents,
            map: self,
        }
    }
}
impl<'a, T: GA> Compose<DeepMappedGA<'a, T>, DeepMappedGA<'a, T>> for DeepMap {
    fn compose(self, contents: DeepMappedGA<T>) -> DeepMappedGA<T> {
        self.compose(contents.map).compose_ref(contents.internal)
    }
}

#[derive(Debug, Clone)]
pub struct DeepMappedGA<'a, T: GA> {
    pub internal: &'a T,
    pub map: DeepMap,
}
impl<'a, T: GA> DeepMappedGA<'a, T> {
    pub fn new(ga: &'a T, map: DeepMap) -> Self {
        Self { internal: ga, map }
    }

    pub fn get_internal(self) -> &'a T {
        self.internal
    }
}
impl<T: GA> GA for DeepMappedGA<'_, T> {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let arr = self.map.map(index);
        self.internal.get(&arr)
    }
}
