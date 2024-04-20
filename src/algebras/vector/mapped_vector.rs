use std::{borrow::Cow, collections::HashMap};

use crate::index_sa::{TryIndexSA, TryIndexSAMut};

use super::Vectorize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MappedVector<'a, T: Vectorize + Clone> {
    vector: Cow<'a, T>,
    map: HashMap<usize, usize>,
}

impl<'a, T: Vectorize + Clone> MappedVector<'a, T> {
    pub fn new(vector: Cow<'a, T>, map: HashMap<usize, usize>) -> Self {
        Self {
            vector,
            map,
        }
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

    //TODO iter() with custom iterator

    //TODO iter_mut() with custom iterator
}

//TODO IntoIterator

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

impl<'a, T: Vectorize + Clone> Vectorize for MappedVector<'a, T> {}
