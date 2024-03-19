use crate::geo_map::{GAMap, MappedGA};

pub trait GA: Sized {
    fn get(&self, index: &[usize]) -> Option<f32>;
    fn map<T: GA>(&self, map: GAMap) -> MappedGA<T>
    where
        Self: AsRef<T>,
    {
        MappedGA::new(self.as_ref(), map)
    }
}
