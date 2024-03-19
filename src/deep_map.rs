use crate::{
    collect::{CollectFromDeep, CollectFromShallow},
    geo_algebra::GA,
    shallow_map::ShallowMappedGA,
};

#[derive(Debug, Clone)]
pub struct DeepMap {}
impl DeepMap {
    pub fn map(&self, index: &[usize]) -> Box<[usize]> {
        todo!()
    }

    pub fn compose(&mut self, other: &Self) {
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

    pub fn collect<U>(&self) -> U
    where
        T: CollectFromDeep<U>,
    {
        T::collect_from_deep(self)
    }
}
impl<T: GA> GA for DeepMappedGA<'_, T> {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let arr = self.map.map(index);
        self.internal.get(&arr)
    }
}
impl<T: GA> CollectFromShallow<Self> for DeepMappedGA<'_, T> {
    fn collect_from_shallow(mapped: &ShallowMappedGA<Self>) -> Self {
        let mut map = mapped.map.to_deep();
        map.compose(&mapped.internal.map);
        Self {
            internal: mapped.internal.internal,
            map,
        }
    }
}
impl<T: GA> CollectFromDeep<Self> for DeepMappedGA<'_, T> {
    fn collect_from_deep(mapped: &DeepMappedGA<Self>) -> Self {
        let mut map = mapped.map.clone();
        map.compose(&mapped.internal.map);
        DeepMappedGA {
            internal: mapped.internal.internal,
            map,
        }
    }
}
