use crate::index_sa::{IndexSA, IndexSAMut, TryIndexSA, TryIndexSAMut};

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
        std::iter::once(&self.x)
            .chain(std::iter::once(&self.y))
            .chain(std::iter::once(&self.z))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        std::iter::once(&mut self.x)
            .chain(std::iter::once(&mut self.y))
            .chain(std::iter::once(&mut self.z))
    }
}

impl IntoIterator for Vector3 {
    type Item = f64;
    type IntoIter = std::iter::Chain<
        std::iter::Chain<std::iter::Once<Self::Item>, std::iter::Once<Self::Item>>,
        std::iter::Once<Self::Item>,
    >;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.x)
            .chain(std::iter::once(self.y))
            .chain(std::iter::once(self.z))
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

impl Vectorize for Vector3 {}
