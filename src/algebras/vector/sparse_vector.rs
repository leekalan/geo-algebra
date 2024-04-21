use std::collections::HashMap;

use crate::{
    enumerate_ga::{EnumerateAndSortGA, EnumerateGA},
    index_ga::{TryIndexGA, TryIndexGAMut},
    iterate_values_ga::IterateValuesGA,
    size_ga::{RangeGA, SizeGA},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SparseVector {
    dimension_map: HashMap<usize, f64>,
}

impl SparseVector {
    pub fn new(dimension_map: HashMap<usize, f64>) -> Self {
        Self { dimension_map }
    }

    pub fn new_empty() -> Self {
        Self::default()
    }

    pub fn from_vector<T: Vectorize>(vector: T) -> Self {
        SparseVector::new(HashMap::from_iter(vector.into_enumerate()))
    }

    pub fn insert(&mut self, dimension: usize, value: f64) -> Option<f64> {
        self.dimension_map.insert(dimension, value)
    }

    pub fn hash_map(&self) -> &HashMap<usize, f64> {
        &self.dimension_map
    }

    pub fn hash_map_mut(&mut self) -> &mut HashMap<usize, f64> {
        &mut self.dimension_map
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &f64)> {
        self.dimension_map.iter().map(|(k, v)| (*k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        self.dimension_map.iter_mut().map(|(k, v)| (*k, v))
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

impl TryIndexGA<usize> for SparseVector {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimension_map.get(&index)
    }
}

impl TryIndexGAMut<usize> for SparseVector {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimension_map.get_mut(&index)
    }
}

impl SizeGA for SparseVector {
    fn size(&self) -> usize {
        self.dimension_map.len()
    }
}
impl RangeGA for SparseVector {
    fn range(&self) -> usize {
        self.dimension_map.keys().max().map(|k| k + 1).unwrap_or(0)
    }
}

impl IterateValuesGA for SparseVector {
    fn iterate_values(&self) -> impl Iterator<Item = &f64> {
        self.dimension_map.values()
    }
    fn iterate_values_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.dimension_map.values_mut()
    }
    fn into_iterate_values(self) -> impl Iterator<Item = f64> {
        self.dimension_map.into_values()
    }
}

impl EnumerateGA<usize> for SparseVector {
    fn enumerate(&self) -> impl Iterator<Item = (usize, &f64)> {
        self.iter()
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        self.iter_mut()
    }
    fn into_enumerate(self) -> impl Iterator<Item = (usize, f64)> {
        self.into_iter()
    }
}
impl EnumerateAndSortGA<usize> for SparseVector {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (usize, &f64)> {
        let mut vec: Vec<_> = self.iter().collect();
        vec.sort_by_key(|(k, _)| *k);
        vec.into_iter()
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        let mut vec: Vec<_> = self.iter_mut().collect();
        vec.sort_by_key(|(k, _)| *k);
        vec.into_iter()
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (usize, f64)> {
        let mut vec: Vec<_> = self.into_iter().collect();
        vec.sort_by_key(|(k, _)| *k);
        vec.into_iter()
    }
}

impl Vectorize for SparseVector {}
