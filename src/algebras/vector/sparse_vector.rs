use std::{borrow::Cow, collections::HashMap};

use crate::index_sa::{TryIndexSA, TryIndexSAMut};

use super::{dynamic_vector, DynamicVector, MappedVector, Vectorize};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SparseVector {
    dimension_map: HashMap<usize, f64>,
}

impl SparseVector {
    pub fn new(dimension_map: HashMap<usize, f64>) -> Self {
        Self { dimension_map }
    }

    pub fn insert(&mut self, dimension: usize, value: f64) {
        self.dimension_map.insert(dimension, value);
    }

    pub fn hash_map(&self) -> &HashMap<usize, f64> {
        &self.dimension_map
    }

    pub fn hash_map_mut(&mut self) -> &mut HashMap<usize, f64> {
        &mut self.dimension_map
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &f64)> {
        self.dimension_map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&usize, &mut f64)> {
        self.dimension_map.iter_mut()
    }

    pub fn into_dynamic_vector(self) -> DynamicVector {
        todo!("fill in the gaps to have a complete vector")
    }

    // TODO
    pub fn into_mapped_dynamic_vector<'a>(self) -> MappedVector<'a, DynamicVector> {
        let dynamic_vector = DynamicVector::default();
        MappedVector::new(Cow::Owned(dynamic_vector), todo!())
    }

    // TODO
    pub fn into_mapped_ref_dynamic_vector(&self) -> MappedVector<'_, DynamicVector> {
        let dynamic_vector = DynamicVector::default();
        MappedVector::new(Cow::Borrowed(&dynamic_vector), todo!())
    }
}

impl AsRef<HashMap<usize, f64>> for SparseVector {
    fn as_ref(&self) -> &HashMap<usize, f64> {
        self.hash_map()
    }
}

impl AsMut<HashMap<usize, f64>> for SparseVector {
    fn as_mut(&mut self) -> &mut HashMap<usize, f64> {
        self.hash_map_mut()
    }
}

impl IntoIterator for SparseVector {
    type Item = (usize, f64);
    type IntoIter = std::collections::hash_map::IntoIter<usize, f64>;
    fn into_iter(self) -> Self::IntoIter {
        self.dimension_map.into_iter()
    }
}

impl TryIndexSA<usize> for SparseVector {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimension_map.get(&index)
    }
}

impl TryIndexSAMut<usize> for SparseVector {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimension_map.get_mut(&index)
    }
}

impl Vectorize for SparseVector {}
