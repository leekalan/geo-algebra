pub mod ops;

use crate::{
    enumerate_ga::{EnumerateAndSortGA, EnumerateGA},
    index_ga::{IndexGA, IndexGAMut, TryIndexGA, TryIndexGAMut},
    iterate_values_ga::IterateValuesGA,
    size_ga::{RangeGA, SizeGA},
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

impl IndexGA<Vector2Index> for Vector2 {
    fn at(&self, index: Vector2Index) -> &f64 {
        match index {
            Vector2Index::X => unsafe { self.dimensions.get_unchecked(0) },
            Vector2Index::Y => unsafe { self.dimensions.get_unchecked(1) },
        }
    }
}

impl IndexGAMut<Vector2Index> for Vector2 {
    fn at_mut(&mut self, index: Vector2Index) -> &mut f64 {
        match index {
            Vector2Index::X => unsafe { self.dimensions.get_unchecked_mut(0) },
            Vector2Index::Y => unsafe { self.dimensions.get_unchecked_mut(1) },
        }
    }
}

impl TryIndexGA<usize> for Vector2 {
    fn try_at(&self, index: usize) -> Option<&f64> {
        self.dimensions.get(index)
    }
}

impl TryIndexGAMut<usize> for Vector2 {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        self.dimensions.get_mut(index)
    }
}

impl SizeGA for Vector2 {
    fn size(&self) -> usize {
        2
    }
}
impl RangeGA for Vector2 {
    fn range(&self) -> usize {
        2
    }
}

impl IterateValuesGA for Vector2 {
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

impl EnumerateGA<Vector2Index> for Vector2 {
    fn enumerate(&self) -> impl Iterator<Item = (Vector2Index, &f64)> {
        EnumerateGA::<usize>::enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector2Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (Vector2Index, &mut f64)> {
        EnumerateGA::<usize>::enumerate_mut(self).map(|(index, data)| {
            (
                unsafe { Vector2Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
    fn into_enumerate(self) -> impl Iterator<Item = (Vector2Index, f64)> {
        EnumerateGA::<usize>::into_enumerate(self).map(|(index, data)| {
            (
                unsafe { Vector2Index::from(index).unwrap_unchecked() },
                data,
            )
        })
    }
}
impl EnumerateAndSortGA<Vector2Index> for Vector2 {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (Vector2Index, &f64)> {
        EnumerateGA::<Vector2Index>::enumerate(self)
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (Vector2Index, &mut f64)> {
        EnumerateGA::<Vector2Index>::enumerate_mut(self)
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (Vector2Index, f64)> {
        EnumerateGA::<Vector2Index>::into_enumerate(self)
    }
}

impl EnumerateGA<usize> for Vector2 {
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
impl EnumerateAndSortGA<usize> for Vector2 {
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

impl Vectorize for Vector2 {}
