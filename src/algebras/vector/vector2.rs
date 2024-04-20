use crate::index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut};

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
        std::iter::once(&self.x).chain(std::iter::once(&self.y))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        std::iter::once(&mut self.x).chain(std::iter::once(&mut self.y))
    }
}

impl IntoIterator for Vector2 {
    type Item = f64;
    type IntoIter = std::iter::Chain<std::iter::Once<Self::Item>, std::iter::Once<Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.x).chain(std::iter::once(self.y))
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

impl Vectorize for Vector2 {}
