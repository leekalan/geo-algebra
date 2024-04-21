pub mod dynamic_vector;
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
    enumerate_ga::EnumerateAndSortGA, index_ga::{TryIndexGA, TryIndexGAMut}, iterate_values_ga::IterateValuesGA, operations::add_ga::{AddGA, AddRefGA}, size_ga::RangeGA
};

pub trait Vectorize:
    Sized + Default + TryIndexGA<usize> + TryIndexGAMut<usize> + RangeGA + IterateValuesGA + EnumerateAndSortGA<usize>
{
    fn generic_vector(self) -> GenericVector<Self> {
        GenericVector { vector: self }
    }

    fn generic_vector_ref(&self) -> GenericVectorRef<Self> {
        GenericVectorRef { vector: self }
    }
}

pub struct GenericVector<T: Vectorize> {
    pub vector: T,
}

pub struct GenericVectorRef<'a, T: Vectorize> {
    vector: &'a T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_sub_test() {
        let a = SparseVector::new([(1, 1.), (2, 2.)].into());
        let b = SparseVector::new([(0, 10.), (1, 11.), (3, 13.)].into());
        assert_eq!(
            a.clone().generic_vector() + b.generic_vector_ref(),
            SparseVector::new([(0, 10.), (1, 12.), (2, 2.), (3, 13.)].into())
        );
        assert_eq!(
            a.clone().generic_vector() + b.generic_vector_ref(),
            SparseVector::new([(0, 10.), (1, 12.), (2, 2.), (3, 13.)].into())
        );
        assert_eq!(
            a.generic_vector() - b,
            SparseVector::new([(0, -10.), (1, -10.), (2, 2.), (3, -13.)].into())
        );
    }
}
