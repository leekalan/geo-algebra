macro_rules! generate_ops_impl {
    ($type:ty) => {
        impl std::ops::Add for $type {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                let mut result = self;
                GeoAlgebra::add(&mut result, rhs);
                result
            }
        }
        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, other: Self) {
                GeoAlgebra::add(self, other);
            }
        }
        
        impl std::ops::Sub for $type {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut result = self;
                GeoAlgebra::sub(&mut result, rhs);
                result
            }
        }
        impl std::ops::SubAssign for $type {
            fn sub_assign(&mut self, other: Self) {
                GeoAlgebra::sub(self, other);
            }
        }
        
        impl std::ops::Mul for $type {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut result = self;
                GeoAlgebra::mul(&mut result, rhs);
                result
            }
        }
        impl std::ops::MulAssign for $type {
            fn mul_assign(&mut self, other: Self) {
                GeoAlgebra::mul(self, other);
            }
        }
    };
}

macro_rules! generate_other_ops_impl {
    ($type:ty) => {
        impl std::ops::Div for $type {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                let mut result = self;
                GeoAlgebraOtherOps::div(&mut result, rhs);
                result
            }
        }
        impl std::ops::DivAssign for $type {
            fn div_assign(&mut self, other: Self) {
                GeoAlgebraOtherOps::div(self, other);
            }
        }
    };
}

pub(crate) use generate_ops_impl;
pub(crate) use generate_other_ops_impl;