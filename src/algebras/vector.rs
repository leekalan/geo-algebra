pub mod dynamic_vector;
pub mod functions;
pub mod generic_ops;
pub mod mapped_vector;
pub mod sparse_vector;
pub mod vector1;
pub mod vector2;
pub mod vector3;
pub mod vector4;

pub use dynamic_vector::DynamicVector;
pub use mapped_vector::MappedVector;
pub use sparse_vector::SparseVector;
pub use vector1::Vector1;
pub use vector2::Vector2;
pub use vector3::Vector3;
pub use vector4::Vector4;

use crate::{
    enumerate_ga::EnumerateAndSortGA,
    index_ga::{TryIndexGA, TryIndexGAMut},
    iterate_values_ga::IterateValuesGA,
    size_ga::RangeGA,
};

use super::scalar::Scalar;

pub trait Vectorize:
    Sized
    + Default
    + TryIndexGA<usize>
    + TryIndexGAMut<usize>
    + RangeGA
    + IterateValuesGA
    + EnumerateAndSortGA<usize>
{
    fn to_sparse(self) -> SparseVector {
        SparseVector::from_vector(self)
    }

    fn to_sparse_ref(&self) -> SparseVector {
        SparseVector::from_vector_ref(self)
    }

    fn gen(self) -> GenericVector<Self> {
        GenericVector { vector: self }
    }

    fn gen_ref(&self) -> GenericVectorRef<Self> {
        GenericVectorRef { vector: self }
    }
}

pub struct GenericVector<T: Vectorize> {
    pub vector: T,
}
impl<T: Vectorize> GenericVector<T> {
    pub fn to_ref(&self) -> GenericVectorRef<T> {
        GenericVectorRef {
            vector: &self.vector,
        }
    }
}

pub struct GenericVectorRef<'a, T: Vectorize> {
    vector: &'a T,
}
