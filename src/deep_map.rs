use crate::{compose::Compose, geo_algebra::GA};

#[derive(Debug, Clone)]
pub struct DeepMap {}
impl DeepMap {
    pub fn map(&self, index: &[usize]) -> Box<[usize]> {
        todo!()
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