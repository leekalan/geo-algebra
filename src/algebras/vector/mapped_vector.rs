use std::{borrow::Cow, collections::HashMap};

use crate::{
    enumerate_sa::{EnumerateAndSortSA, EnumerateSA},
    index_sa::{TryIndexSA, TryIndexSAMut},
    size_sa::{RangeSA, SizeSA},
};

use super::Vectorize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MappedVector<'a, T: Vectorize + Clone> {
    vector: Cow<'a, T>,
    map: HashMap<usize, usize>,
}

impl<'a, T: Vectorize + Clone> MappedVector<'a, T> {
    pub fn new(vector: Cow<'a, T>, map: HashMap<usize, usize>) -> Self {
        Self { vector, map }
    }

    pub fn map(&self) -> &HashMap<usize, usize> {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut HashMap<usize, usize> {
        &mut self.map
    }

    pub fn unmapped_vector(&self) -> &T {
        &self.vector
    }

    pub fn unmapped_vector_mut(&mut self) -> &mut T {
        self.vector.to_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &f64)> {
        self.vector
            .as_ref()
            .enumerate()
            .map(|(k, v)| (*self.map.get(&k).unwrap_or(&k), v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        self.vector
            .to_mut()
            .enumerate_mut()
            .map(|(k, v)| (*self.map.get(&k).unwrap_or(&k), v))
    }
}

pub struct MappedVectorIter<'a> {
    iter: Box<dyn Iterator<Item = (usize, f64)> + 'a>,
    map: HashMap<usize, usize>,
}
impl Iterator for MappedVectorIter<'_> {
    type Item = (usize, f64);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(k, v)| (*self.map.get(&k).unwrap_or(&k), v))
    }
}
impl<'a, T: Vectorize + Clone> IntoIterator for MappedVector<'a, T> {
    type Item = (usize, f64);
    type IntoIter = MappedVectorIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        let heap_iter = Box::new(self.vector.into_owned().into_enumerate());
        MappedVectorIter {
            iter: heap_iter,
            map: self.map,
        }
    }
}

impl<'a, T: Vectorize + Clone> TryIndexSA<usize> for MappedVector<'a, T> {
    fn try_at(&self, index: usize) -> Option<&f64> {
        let mapped_index = *self.map.get(&index)?;
        self.vector.try_at(mapped_index)
    }
}

impl<'a, T: Vectorize + Clone> TryIndexSAMut<usize> for MappedVector<'a, T> {
    fn try_at_mut(&mut self, index: usize) -> Option<&mut f64> {
        let mapped_index = *self.map.get(&index)?;
        self.vector.to_mut().try_at_mut(mapped_index)
    }
}

impl<T: Vectorize + Clone + SizeSA> SizeSA for MappedVector<'_, T> {
    fn size(&self) -> usize {
        self.vector.as_ref().size()
    }
}

impl<T: Vectorize + Clone> RangeSA for MappedVector<'_, T> {
    fn range(&self) -> usize {
        self.vector
            .as_ref()
            .enumerate()
            .map(|(k, _)| *self.map.get(&k).unwrap_or(&k))
            .max()
            .map(|k| k + 1)
            .unwrap_or(0)
    }
}

impl<T: Vectorize + Clone> EnumerateSA<usize> for MappedVector<'_, T> {
    fn enumerate(&self) -> impl Iterator<Item = (usize, &f64)> {
        self.iter()
    }
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        self.iter_mut()
    }
    fn into_enumerate(self) -> impl Iterator<Item = (usize, f64)> {
        self.into_iter()
    }
}
impl<T: Vectorize + Clone> EnumerateAndSortSA<usize> for MappedVector<'_, T> {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (usize, &f64)> {
        let mut vec: Vec<_> = self.enumerate().collect();
        vec.sort_by_key(|(k, _)| *k);
        vec.into_iter()
    }
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (usize, &mut f64)> {
        let mut vec: Vec<_> = self.enumerate_mut().collect();
        vec.sort_by_key(|(k, _)| *k);
        vec.into_iter()
    }
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (usize, f64)> {
        let mut vec: Vec<_> = self.into_enumerate().collect();
        vec.sort_by_key(|(k, _)| *k);
        vec.into_iter()
    }
}

impl<'a, T: Vectorize + Clone> Vectorize for MappedVector<'a, T> {}
