use crate::{
    enumerate_ga::{EnumerateAndSortGA, EnumerateGA},
    index_ga::{IndexGA, IndexGAMut, TryIndexGA, TryIndexGAMut},
    iterate_values_ga::IterateValuesGA,
    size_ga::{RangeGA, SizeGA},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector4 {
    dimensions: [f64; 4],
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            dimensions: [x, y, z, w],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.dimensions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.dimensions.iter_mut()
    }
}

impl IntoIterator for Vector4 {
    type Item = f64;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;
    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector4Index {
    X,
    Y,
    Z,
    W,
}
impl Vector4Index {
    pub fn from(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::X),
            1 => Some(Self::Y),
            2 => Some(Self::Z),
            3 => Some(Self::W),
            _ => None,
        }
    }
}

impl IndexGA<Vector4Index> for Vector4 {
    fn at(&self, index: Vector4Index) -> &f64 {
        match index {
            Vector4Index::X => unsafe { self.dimensions.get_unchecked(0) },
            Vector4Index::Y => unsafe { self.dimensions.get_unchecked(1) },
            Vector4Index::Z => unsafe { self.dimensions.get_unchecked(2) },
            Vector4Index::W => unsafe { self.dimensions.get_unchecked(3) },
        }
    }
}

impl IndexGAMut<Vector4Index> for Vector4 {
    fn at_mut(&mut self, index: Vector4Index) -> &mut f64 {
        match index {
            Vector4Index::X => unsafe { self.dimensions.get_unchecked_mut(0) },
            Vector4Index::Y => unsafe { self.dimensions.get_unchecked_mut(1) },
            Vector4Index::Z => unsafe { self.dimensions.get_unchecked_mut(2) },
            Vector4Index::W => unsafe { self.dimensions.get_unchecked_mut(3) },
        }
    }
}

impl TryIndexGA<usize> for Vector4 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimensions.get(index)
    }
}

impl TryIndexGAMut<usize> for Vector4 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimensions.get_mut(index)
    }
}

impl SizeGA for Vector4 {
    fn size(&self) -> usize {
        4
    }
}
impl RangeGA for Vector4 {
    fn range(&self) -> usize {
        4
    }
}

impl IterateValuesGA for Vector4 {
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

impl EnumerateGA<Vector4Index> for Vector4 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector4Index, &f64)> {
        EnumerateGA::<usize>::enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector4Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector4Index, &mut f64)> {
        EnumerateGA::<usize>::enumerate_mut(self).map(|(index, data)| {
            (
                unsafe { Vector4Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn into_enumerate(self) -> impl Iterator<Item = (Vector4Index, f64)> {
        EnumerateGA::<usize>::into_enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector4Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
}
impl EnumerateAndSortGA<Vector4Index> for Vector4 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (Vector4Index, &f64)> {
        EnumerateGA::<Vector4Index>::enumerate(self)
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (Vector4Index, &mut f64)> {
        EnumerateGA::<Vector4Index>::enumerate_mut(self)
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (Vector4Index, f64)> {
        EnumerateGA::<Vector4Index>::into_enumerate(self)
    }
}

impl EnumerateGA<usize> for Vector4 {
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
impl EnumerateAndSortGA<usize> for Vector4 {
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

impl Vectorize for Vector4 {}
