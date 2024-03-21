use crate::{collect::Compose, geo_algebra::GA};

#[derive(Debug, Clone)]
pub struct DeepMap {}
impl DeepMap {
    pub fn map(&self, index: &[usize]) -> Box<[usize]> {
        todo!()
    }
}
impl Compose<DeepMap> for DeepMap {
    fn compose(&mut self, other: Self) {
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

    pub fn compose(&mut self, map: DeepMap) {
        let mut map = map;
        std::mem::swap(&mut self.map, &mut map);
        self.map.compose(map);
    }
}
impl<T: GA> GA for DeepMappedGA<'_, T> {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let arr = self.map.map(index);
        self.internal.get(&arr)
    }
}