use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{TryIndexSA, TryIndexSAMut},
    size_sa::{RangeSA, SizeSA},
};

use super::{SparseVector, Vectorize};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DynamicVector {
    dimensions: Vec<f64>,
}

impl DynamicVector {
    pub fn new(dimensions: Vec<f64>) -> Self {
        Self { dimensions }
    }

    pub fn new_empty() -> Self {
        Self::default()
    }

    pub fn from_sparse_vector(sparse_vector: SparseVector) -> Self {
        let mut vec = Vec::with_capacity(sparse_vector.range());

        for (dimension, value) in sparse_vector.into_enumerate_and_sort() {
            vec.resize(dimension, 0f64);
            vec.push(value)
        }

        Self { dimensions: vec }
    }

    pub fn from_vector(vector: impl Vectorize) -> Self {
        Self::from_sparse_vector(SparseVector::from_vector(vector))
    }

    pub fn push(&mut self, dimension: f64) {
        self.dimensions.push(dimension)
    }

    pub fn extend(&mut self, iter: impl Iterator<Item = f64>) {
        self.dimensions.extend(iter)
    }

    pub fn slice(&self) -> &[f64] {
        &self.dimensions
    }

    pub fn mut_vec(&mut self) -> &mut Vec<f64> {
        &mut self.dimensions
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.dimensions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.dimensions.iter_mut()
    }
}

impl AsRef<[f64]> for DynamicVector {
    fn as_ref(&self) -> &[f64] {
        self.slice()
    }
}

impl AsMut<Vec<f64>> for DynamicVector {
    fn as_mut(&mut self) -> &mut Vec<f64> {
        &mut self.dimensions
    }
}

impl IntoIterator for DynamicVector {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

impl TryIndexSA<usize> for DynamicVector {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimensions.get(index)
    }
}

impl TryIndexSAMut<usize> for DynamicVector {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimensions.get_mut(index)
    }
}

impl SizeSA for DynamicVector {
    fn size(&self) -> usize {
        self.dimensions.len()
    }
}
impl RangeSA for DynamicVector {
    fn range(&self) -> usize {
        self.dimensions.len()
    }
}

impl EnumerateSA<usize> for DynamicVector {
    fn enumerate(&self) -> impl Iterator<Item = (usize, &f64)> {
        self.iter().enumerate()
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        self.iter_mut().enumerate()
    }
    fn into_enumerate(self) -> impl Iterator<Item = (usize, f64)> {
        self.into_iter().enumerate()
    }
}
impl EnumerateAndSortSA<usize> for DynamicVector {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (usize, &f64)> {
        self.enumerate()
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        self.enumerate_mut()
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (usize, f64)> {
        self.into_enumerate()
    }
}

impl Vectorize for DynamicVector {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_extend_sparse_vector() {
        let sparse_vector = SparseVector::new(HashMap::from([
            (2, 3.),
            (4, 7.),
        ]));
        let vector = DynamicVector::from_sparse_vector(sparse_vector);
        assert_eq!(5, vector.size());
        assert_eq!(5, vector.range());
        assert_eq!(Some(&0.), vector.try_at(0));
        assert_eq!(Some(&0.), vector.try_at(1));
        assert_eq!(Some(&3.), vector.try_at(2));
        assert_eq!(Some(&0.), vector.try_at(3));
        assert_eq!(Some(&7.), vector.try_at(4));
        assert_eq!(None, vector.try_at(5));
    }


    #[test]
    fn test_sort_dynamic_vector() {
        let vector = DynamicVector::new(vec![3., 1., 2.]);
        let sorted_vector = vector.enumerate().collect::<Vec<_>>();
        assert_eq!(sorted_vector, vec![(0, &3.), (1, &1.), (2, &2.)]);
    }
}
