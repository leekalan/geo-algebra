use crate::{
    enumerate_ga::{EnumerateAndSortGA, EnumerateGA},
    index_ga::{IndexGA, IndexGAMut, TryIndexGA, TryIndexGAMut},
    iterate_values_ga::IterateValuesGA,
    size_ga::{RangeGA, SizeGA},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector1 {
    pub dimensions: [f64; 1],
}

impl Vector1 {
    pub fn new(x: f64) -> Self {
        Self { dimensions: [x] }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.dimensions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.dimensions.iter_mut()
    }
}

impl AsRef<f64> for Vector1 {
    fn as_ref(&self) -> &f64 {
        self.at(Vector1Index::X)
    }
}

impl AsMut<f64> for Vector1 {
    fn as_mut(&mut self) -> &mut f64 {
        self.at_mut(Vector1Index::X)
    }
}

impl IntoIterator for Vector1 {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 1>;
    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector1Index {
    X,
}
impl Vector1Index {
    pub fn from(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::X),
            _ => None,
        }
    }
}

impl IndexGA<Vector1Index> for Vector1 {
    fn at(&self, index: Vector1Index) -> &f64 {
        match index {
            Vector1Index::X => unsafe { self.dimensions.get_unchecked(0) },
        }
    }
}

impl IndexGAMut<Vector1Index> for Vector1 {
    fn at_mut(&mut self, index: Vector1Index) -> &mut f64 {
        match index {
            Vector1Index::X => unsafe { self.dimensions.get_unchecked_mut(0) },
        }
    }
}

impl TryIndexGA<usize> for Vector1 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimensions.get(index)
    }
}

impl TryIndexGAMut<usize> for Vector1 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimensions.get_mut(index)
    }
}

impl SizeGA for Vector1 {
    fn size(&self) -> usize {
        1
    }
}
impl RangeGA for Vector1 {
    fn range(&self) -> usize {
        1
    }
}

impl IterateValuesGA for Vector1 {
    fn iterate_values(&self) -> impl Iterator<Item = &f64> {
        self.iter()
    }
    fn iterate_values_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.iter_mut()
    }
    fn into_iterate_values(self) -> impl Iterator<Item = f64> {
        self.into_iter()
    }
}

impl EnumerateGA<Vector1Index> for Vector1 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector1Index, &f64)> {
        self.iter().map(|x| (Vector1Index::X, x))
    }

    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector1Index, &mut f64)> {
        self.iter_mut().map(|x| (Vector1Index::X, x))
    }

    fn into_enumerate(self) -> impl Iterator<Item = (Vector1Index, f64)> {
        self.into_iter().map(|x| (Vector1Index::X, x))
    }
}
impl EnumerateAndSortGA<Vector1Index> for Vector1 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (Vector1Index, &f64)> {
        EnumerateGA::<Vector1Index>::enumerate(self)
    }

    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (Vector1Index, &mut f64)> {
        EnumerateGA::<Vector1Index>::enumerate_mut(self)
    }

    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (Vector1Index, f64)> {
        EnumerateGA::<Vector1Index>::into_enumerate(self)
    }
}

impl EnumerateGA<usize> for Vector1 {
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
impl EnumerateAndSortGA<usize> for Vector1 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (usize, &f64)> {
        EnumerateGA::<usize>::enumerate(self)
    }

    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        EnumerateGA::<usize>::enumerate_mut(self)
    }

    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (usize, f64)> {
        EnumerateGA::<usize>::into_enumerate(self)
    }
}

impl Vectorize for Vector1 {}
