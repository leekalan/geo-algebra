use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        [&self.x, &self.y].into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        [&mut self.x, &mut self.y].into_iter()
    }
}

impl IntoIterator for Vector2 {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 2>;
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector2Index {
    X,
    Y,
}

impl IndexSA<Vector2Index> for Vector2 {
    fn at(&self, index: Vector2Index) -> &f64 {
        match index {
            Vector2Index::X => &self.x,
            Vector2Index::Y => &self.y,
        }
    }
}

impl IndexSAMut<Vector2Index> for Vector2 {
    fn at_mut(&mut self, index: Vector2Index) -> &mut f64 {
        match index {
            Vector2Index::X => &mut self.x,
            Vector2Index::Y => &mut self.y,
        }
    }
}

impl TryIndexSA<usize> for Vector2 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        match index {
            0 => Some(&self.x),
            1 => Some(&self.y),
            _ => None,
        }
    }
}

impl TryIndexSAMut<usize> for Vector2 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        match index {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            _ => None,
        }
    }
}

impl EnumerateSA<Vector2Index> for Vector2 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector2Index, &f64)> {
        [(Vector2Index::X, &self.x), (Vector2Index::Y, &self.y)].into_iter()
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector2Index, &mut f64)> {
        [
            (Vector2Index::X, &mut self.x),
            (Vector2Index::Y, &mut self.y),
        ]
        .into_iter()
    }
    fn into_enumerate(self) -> impl Iterator<Item = (Vector2Index, f64)> {
        [(Vector2Index::X, self.x), (Vector2Index::Y, self.y)].into_iter()
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
