use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector1 {
    pub x: f64,
}

impl Vector1 {
    pub fn new(x: f64) -> Self {
        Self { x }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        std::iter::once(&self.x)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        std::iter::once(&mut self.x)
    }
}

impl AsRef<f64> for Vector1 {
    fn as_ref(&self) -> &f64 {
        &self.x
    }
}

impl AsMut<f64> for Vector1 {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.x
    }
}

impl IntoIterator for Vector1 {
    type Item = f64;
    type IntoIter = std::iter::Once<f64>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector1Index {
    X,
}

impl IndexSA<Vector1Index> for Vector1 {
    fn at(&self, index: Vector1Index) -> &f64 {
        match index {
            Vector1Index::X => &self.x,
        }
    }
}

impl IndexSAMut<Vector1Index> for Vector1 {
    fn at_mut(&mut self, index: Vector1Index) -> &mut f64 {
        match index {
            Vector1Index::X => &mut self.x,
        }
    }
}

impl TryIndexSA<usize> for Vector1 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        match index {
            0 => Some(&self.x),
            _ => None,
        }
    }
}

impl TryIndexSAMut<usize> for Vector1 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        match index {
            0 => Some(&mut self.x),
            _ => None,
        }
    }
}

impl EnumerateSA<Vector1Index> for Vector1 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector1Index, &f64)> {
        std::iter::once((Vector1Index::X, &self.x))
    }

    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector1Index, &mut f64)> {
        std::iter::once((Vector1Index::X, &mut self.x))
    }

    fn into_enumerate(self) -> impl Iterator<Item = (Vector1Index, f64)> {
        std::iter::once((Vector1Index::X, self.x))
    }
}
impl EnumerateAndSortSA<Vector1Index> for Vector1 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (Vector1Index, &f64)> {
        EnumerateSA::<Vector1Index>::enumerate(self)
    }

    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (Vector1Index, &mut f64)> {
        EnumerateSA::<Vector1Index>::enumerate_mut(self)
    }

    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (Vector1Index, f64)> {
        EnumerateSA::<Vector1Index>::into_enumerate(self)
    }
}

impl EnumerateSA<usize> for Vector1 {
    fn enumerate(&self) -> impl Iterator<Item = (usize, &f64)> {
        std::iter::once((0, &self.x))
    }

    fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        std::iter::once((1, &mut self.x))
    }

    fn into_enumerate(self) -> impl Iterator<Item = (usize, f64)> {
        std::iter::once((2, self.x))
    }
}
impl EnumerateAndSortSA<usize> for Vector1 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (usize, &f64)> {
        EnumerateSA::<usize>::enumerate(self)
    }

    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        EnumerateSA::<usize>::enumerate_mut(self)
    }

    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (usize, f64)> {
        EnumerateSA::<usize>::into_enumerate(self)
    }
}

impl Vectorize for Vector1 {}
