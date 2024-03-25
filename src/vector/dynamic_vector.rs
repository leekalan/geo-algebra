use std::collections::HashMap;

use crate::{geo_algebra::{dot_product::DotProduct, create_shallow_map::CreateShallowMap, sum::Sum, GA}, shallow_map::{ShallowMap, ShallowMappedGA}};

use super::Vector;

pub struct DynamicVector {
    data: HashMap<usize, f32>,
}

impl GA for DynamicVector {
    fn get_multi(&self, index: &[usize]) -> Option<f32> {
        if let &[item] = index {
            self.data.get(&item).copied()
        } else {
            None
        }
    }
}

impl<'a> CreateShallowMap<'a> for DynamicVector {
    type Result = Self;

    fn create_shallow_map(&'a self, map: ShallowMap) -> Self::Result {
        
    }
}

impl<T: Vector> DotProduct<T> for DynamicVector {
    fn dot_product(&self, other: impl std::borrow::Borrow<T>) -> f32 {
        todo!()
    }
}

impl<T: Vector> Sum<T> for DynamicVector {
    type Result = DynamicVector;

    fn sum(&self, other: impl std::borrow::Borrow<T>) -> Self::Result {
        todo!()
    }
}

impl Vector for DynamicVector {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<f32> {
        self.data.get(&index).copied()
    }
}
