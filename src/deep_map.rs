use std::borrow::Borrow;

use wrappr::{
    composer::Composer,
    wrapper::{Wrapper, WrapperRef},
};

use crate::geo_algebra::GA;

#[derive(Debug, Clone)]
pub struct DeepMap {}
impl DeepMap {
    pub fn map(&self, index: &[usize]) -> Box<[usize]> {
        todo!()
    }
}
impl Composer<DeepMap> for DeepMap {
    fn compose(&mut self, contents: DeepMap) -> &mut Self {
        todo!()
    }
}
impl<'a, G: GA> WrapperRef<'a, G, DeepMappedGA<'a, G>> for DeepMap {
    fn wrap_ref(self, contents: &'a impl Borrow<G>) -> DeepMappedGA<'a, G> {
        DeepMappedGA {
            internal: contents.borrow(),
            map: self,
        }
    }
}
impl<'a, T: GA> Wrapper<DeepMappedGA<'a, T>, DeepMappedGA<'a, T>> for DeepMap {
    fn wrap(mut self, contents: DeepMappedGA<T>) -> DeepMappedGA<T> {
        self.compose(contents.map);
        self.wrap_ref(contents.internal)
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
