use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        [&self.x, &self.y, &self.z].into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        [&mut self.x, &mut self.y, &mut self.z].into_iter()
    }
}

impl IntoIterator for Vector3 {
    type Item = f64;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector3Index {
    X,
    Y,
    Z,
}

impl IndexSA<Vector3Index> for Vector3 {
    fn at(&self, index: Vector3Index) -> &f64 {
        match index {
            Vector3Index::X => &self.x,
            Vector3Index::Y => &self.y,
            Vector3Index::Z => &self.z,
        }
    }
}

impl IndexSAMut<Vector3Index> for Vector3 {
    fn at_mut(&mut self, index: Vector3Index) -> &mut f64 {
        match index {
            Vector3Index::X => &mut self.x,
            Vector3Index::Y => &mut self.y,
            Vector3Index::Z => &mut self.z,
        }
    }
}

impl TryIndexSA<usize> for Vector3 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        match index {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None,
        }
    }
}

impl TryIndexSAMut<usize> for Vector3 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        match index {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            _ => None,
        }
    }
}

impl EnumerateSA<Vector3Index> for Vector3 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector3Index, &f64)> {
        [
            (Vector3Index::X, &self.x),
            (Vector3Index::Y, &self.y),
            (Vector3Index::Z, &self.z),
        ]
        .into_iter()
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector3Index, &mut f64)> {
        [
            (Vector3Index::X, &mut self.x),
            (Vector3Index::Y, &mut self.y),
            (Vector3Index::Z, &mut self.z),
        ]
        .into_iter()
    }
    fn into_enumerate(self) -> impl Iterator<Item = (Vector3Index, f64)> {
        [
            (Vector3Index::X, self.x),
            (Vector3Index::Y, self.y),
            (Vector3Index::Z, self.z),
        ]
        .into_iter()
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
