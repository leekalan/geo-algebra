use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut},
    size_sa::{RangeSA, SizeSA},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3 {
    dimensions: [f64; 3],
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            dimensions: [x, y, z],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.dimensions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.dimensions.iter_mut()
    }
}

impl IntoIterator for Vector3 {
    type Item = f64;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;
    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector3Index {
    X,
    Y,
    Z,
}
impl Vector3Index {
    pub fn from(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::X),
            1 => Some(Self::Y),
            2 => Some(Self::Z),
            _ => None,
        }
    }
}

impl IndexSA<Vector3Index> for Vector3 {
    fn at(&self, index: Vector3Index) -> &f64 {
        match index {
            Vector3Index::X => unsafe { self.dimensions.get_unchecked(0) },
            Vector3Index::Y => unsafe { self.dimensions.get_unchecked(1) },
            Vector3Index::Z => unsafe { self.dimensions.get_unchecked(2) },
        }
    }
}

impl IndexSAMut<Vector3Index> for Vector3 {
    fn at_mut(&mut self, index: Vector3Index) -> &mut f64 {
        match index {
            Vector3Index::X => unsafe { self.dimensions.get_unchecked_mut(0) },
            Vector3Index::Y => unsafe { self.dimensions.get_unchecked_mut(1) },
            Vector3Index::Z => unsafe { self.dimensions.get_unchecked_mut(2) },
        }
    }
}

impl TryIndexSA<usize> for Vector3 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimensions.get(index)
    }
}

impl TryIndexSAMut<usize> for Vector3 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimensions.get_mut(index)
    }
}

impl SizeSA for Vector3 {
    fn size(&self) -> usize {
        3
    }
}
impl RangeSA for Vector3 {
    fn range(&self) -> usize {
        3
    }
}

impl EnumerateSA<Vector3Index> for Vector3 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector3Index, &f64)> {
        EnumerateSA::<usize>::enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector3Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector3Index, &mut f64)> {
        EnumerateSA::<usize>::enumerate_mut(self).map(|(index, data)| {
            (
                unsafe { Vector3Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn into_enumerate(self) -> impl Iterator<Item = (Vector3Index, f64)> {
        EnumerateSA::<usize>::into_enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector3Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
}
impl EnumerateAndSortSA<Vector3Index> for Vector3 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (Vector3Index, &f64)> {
        EnumerateSA::<Vector3Index>::enumerate(self)
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (Vector3Index, &mut f64)> {
        EnumerateSA::<Vector3Index>::enumerate_mut(self)
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (Vector3Index, f64)> {
        EnumerateSA::<Vector3Index>::into_enumerate(self)
    }
}

impl EnumerateSA<usize> for Vector3 {
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
impl EnumerateAndSortSA<usize> for Vector3 {
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

impl Vectorize for Vector3 {}
