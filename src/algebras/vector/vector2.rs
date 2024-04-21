use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut},
    size_sa::{RangeSA, SizeSA},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector2 {
    dimensions: [f64; 2],
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { dimensions: [x, y] }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.dimensions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.dimensions.iter_mut()
    }
}

impl IntoIterator for Vector2 {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 2>;
    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector2Index {
    X,
    Y,
}
impl Vector2Index {
    pub fn from(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::X),
            1 => Some(Self::Y),
            _ => None,
        }
    }
}

impl IndexSA<Vector2Index> for Vector2 {
    fn at(&self, index: Vector2Index) -> &f64 {
        match index {
            Vector2Index::X => unsafe { self.dimensions.get_unchecked(0) },
            Vector2Index::Y => unsafe { self.dimensions.get_unchecked(1) },
        }
    }
}

impl IndexSAMut<Vector2Index> for Vector2 {
    fn at_mut(&mut self, index: Vector2Index) -> &mut f64 {
        match index {
            Vector2Index::X => unsafe { self.dimensions.get_unchecked_mut(0) },
            Vector2Index::Y => unsafe { self.dimensions.get_unchecked_mut(1) },
        }
    }
}

impl TryIndexSA<usize> for Vector2 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimensions.get(index)
    }
}

impl TryIndexSAMut<usize> for Vector2 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimensions.get_mut(index)
    }
}

impl SizeSA for Vector2 {
    fn size(&self) -> usize {
        2
    }
}
impl RangeSA for Vector2 {
    fn range(&self) -> usize {
        2
    }
}

impl EnumerateSA<Vector2Index> for Vector2 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector2Index, &f64)> {
        EnumerateSA::<usize>::enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector2Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector2Index, &mut f64)> {
        EnumerateSA::<usize>::enumerate_mut(self).map(|(index, data)| {
            (
                unsafe { Vector2Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn into_enumerate(self) -> impl Iterator<Item = (Vector2Index, f64)> {
        EnumerateSA::<usize>::into_enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector2Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
}
impl EnumerateAndSortSA<Vector2Index> for Vector2 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (Vector2Index, &f64)> {
        EnumerateSA::<Vector2Index>::enumerate(self)
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (Vector2Index, &mut f64)> {
        EnumerateSA::<Vector2Index>::enumerate_mut(self)
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (Vector2Index, f64)> {
        EnumerateSA::<Vector2Index>::into_enumerate(self)
    }
}

impl EnumerateSA<usize> for Vector2 {
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
impl EnumerateAndSortSA<usize> for Vector2 {
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

impl Vectorize for Vector2 {}
