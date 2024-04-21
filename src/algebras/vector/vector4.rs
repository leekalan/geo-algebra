use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        [&self.x, &self.y, &self.z, &self.w].into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        [&mut self.x, &mut self.y, &mut self.z, &mut self.w].into_iter()
    }
}

impl IntoIterator for Vector4 {
    type Item = f64;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z, self.w].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector4Index {
    X,
    Y,
    Z,
    W,
}

impl IndexSA<Vector4Index> for Vector4 {
    fn at(&self, index: Vector4Index) -> &f64 {
        match index {
            Vector4Index::X => &self.x,
            Vector4Index::Y => &self.y,
            Vector4Index::Z => &self.z,
            Vector4Index::W => &self.w,
        }
    }
}

impl IndexSAMut<Vector4Index> for Vector4 {
    fn at_mut(&mut self, index: Vector4Index) -> &mut f64 {
        match index {
            Vector4Index::X => &mut self.x,
            Vector4Index::Y => &mut self.y,
            Vector4Index::Z => &mut self.z,
            Vector4Index::W => &mut self.w,
        }
    }
}

impl TryIndexSA<usize> for Vector4 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        match index {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            3 => Some(&self.w),
            _ => None,
        }
    }
}

impl TryIndexSAMut<usize> for Vector4 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        match index {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            3 => Some(&mut self.w),
            _ => None,
        }
    }
}

impl EnumerateSA<Vector4Index> for Vector4 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector4Index, &f64)> {
        [
            (Vector4Index::X, &self.x),
            (Vector4Index::Y, &self.y),
            (Vector4Index::Z, &self.z),
            (Vector4Index::W, &self.w),
        ]
        .into_iter()
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector4Index, &mut f64)> {
        [
            (Vector4Index::X, &mut self.x),
            (Vector4Index::Y, &mut self.y),
            (Vector4Index::Z, &mut self.z),
            (Vector4Index::W, &mut self.w),
        ]
        .into_iter()
    }
    fn into_enumerate(self) -> impl Iterator<Item = (Vector4Index, f64)> {
        [
            (Vector4Index::X, self.x),
            (Vector4Index::Y, self.y),
            (Vector4Index::Z, self.z),
            (Vector4Index::W, self.w),
        ]
        .into_iter()
    }
}
impl EnumerateAndSortSA<Vector4Index> for Vector4 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (Vector4Index, &f64)> {
        EnumerateSA::<Vector4Index>::enumerate(self)
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (Vector4Index, &mut f64)> {
        EnumerateSA::<Vector4Index>::enumerate_mut(self)
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (Vector4Index, f64)> {
        EnumerateSA::<Vector4Index>::into_enumerate(self)
    }
}

impl EnumerateSA<usize> for Vector4 {
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
impl EnumerateAndSortSA<usize> for Vector4 {
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

impl Vectorize for Vector4 {}
