use std::{borrow::Cow, collections::HashMap};

use crate::algebras::vector::{
    vector2::Vector2Index, vector3::Vector3Index, vector4::Vector4Index, DynamicVector,
    MappedVector, Vector2, Vector3, Vector4,
};

use super::*;

impl Vector1 {
    pub fn to_vector2(&self, index: Vector2Index) -> Vector2 {
        match index {
            Vector2Index::X => Vector2::new(*self.x(), 0.0),
            Vector2Index::Y => Vector2::new(0.0, *self.x()),
        }
    }

    pub fn to_vector3(&self, index: Vector3Index) -> Vector3 {
        match index {
            Vector3Index::X => Vector3::new(*self.x(), 0.0, 0.0),
            Vector3Index::Y => Vector3::new(0.0, *self.x(), 0.0),
            Vector3Index::Z => Vector3::new(0.0, 0.0, *self.x()),
        }
    }

    pub fn to_vector4(&self, index: Vector4Index) -> Vector4 {
        match index {
            Vector4Index::X => Vector4::new(*self.x(), 0.0, 0.0, 0.0),
            Vector4Index::Y => Vector4::new(0.0, *self.x(), 0.0, 0.0),
            Vector4Index::Z => Vector4::new(0.0, 0.0, *self.x(), 0.0),
            Vector4Index::W => Vector4::new(0.0, 0.0, 0.0, *self.x()),
        }
    }

    pub fn to_dynamic_vector(&self, dimension: usize) -> DynamicVector {
        let mut vec = Vec::with_capacity(dimension + 1);
        vec.resize(dimension, 0.);
        vec.push(*self.x());
        DynamicVector::new(vec)
    }

    pub fn mapped<'a>(self, dimension: usize) -> MappedVector<'a, Self> {
        MappedVector::new(Cow::Owned(self), HashMap::from([(dimension, 0)]))
    }
}

#[test]
fn test() {
    let v: Vec<u8> = vec![];
    v.get(0..0).unwrap();
    let v: Vec<u8> = vec![1, 2, 3];
    v.get(0..0).unwrap();
}
