pub mod dynamic_vector;
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
    enumerate_sa::EnumerateAndSortSA,
    index_sa::{TryIndexSA, TryIndexSAMut},
    size_sa::RangeSA,
};

pub trait Vectorize:
    TryIndexSA<usize> + TryIndexSAMut<usize> + RangeSA + EnumerateAndSortSA<usize>
{
}
