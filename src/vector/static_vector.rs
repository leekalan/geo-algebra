use wrappr::injectable::InjectableRef;

use crate::{geo_algebra::{dot_product::DotProduct, create_shallow_map::CreateShallowMap, sum::Sum, GA}, shallow_map::{ShallowMap, ShallowMappedGA}};

use super::Vector;

pub struct StaticVector {
    data: Box<[f32]>,
}
impl StaticVector {
    /// # Safety
    ///
    /// The index must be within bounds of the vector
    unsafe fn get_unchecked(&self, index: usize) -> f32 {
        *self.data.get_unchecked(index)
    }
}

impl GA for StaticVector {
    fn get_multi(&self, index: &[usize]) -> Option<f32> {
        if let &[item] = index {
            self.data.get(item).copied()
        } else {
            None
        }
    }
}

impl<'a> CreateShallowMap<'a> for StaticVector {
    type Result = ShallowMappedGA<'a, StaticVector>;

    fn create_shallow_map(&'a self, map: ShallowMap) -> Self::Result {
        self.inject_ref(map)
    }
}

impl<T: Vector> DotProduct<T> for StaticVector {
    fn dot_product(&self, other: impl std::borrow::Borrow<T>) -> f32 {
        todo!()
    }
}

impl<T: Vector> Sum<T> for StaticVector {
    type Result = StaticVector;

    fn sum(&self, other: impl std::borrow::Borrow<T>) -> Self::Result {
        todo!()
    }
}

impl Vector for StaticVector {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<f32> {
        self.data.get(index).copied()
    }
}
