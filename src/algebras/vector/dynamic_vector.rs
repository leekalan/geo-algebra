use crate::{enumerate_sa::{EnumerateAndSortSA, EnumerateSA}, index_sa::{TryIndexSA, TryIndexSAMut}};

use super::Vectorize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DynamicVector {
    dimensions: Vec<f64>,
}

impl DynamicVector {
    pub fn new(dimensions: Vec<f64>) -> Self {
        Self { dimensions }
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
