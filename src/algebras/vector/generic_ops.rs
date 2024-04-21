use super::*;

mod add {
    use super::*;

    impl<T: Vectorize, U: Vectorize> AddGA<U> for GenericVector<T> {
        type Output = SparseVector;
        fn add_ga(self, other: U) -> SparseVector {
            let mut this = SparseVector::from_vector(self.vector);
            let iter = other.into_enumerate();
            for (dimension, value) in iter {
                if let Some(current) = this.try_at_mut(dimension) {
                    *current += value
                } else {
                    this.insert(dimension, value);
                }
            }
            this
        }
    }
    impl<T: Vectorize, U: Vectorize> std::ops::Add<U> for GenericVector<T> {
        type Output = SparseVector;
        fn add(self, rhs: U) -> Self::Output {
            self.add_ga(rhs)
        }
    }

    impl<T: Vectorize, U: Vectorize> AddRefGA<GenericVectorRef<'_, U>> for GenericVector<T> {
        type Output = SparseVector;
        fn add_ref_ga(self, other: &GenericVectorRef<U>) -> SparseVector {
            let mut this = SparseVector::from_vector(self.vector);
            let iter = other.vector.enumerate();
            for (dimension, value) in iter {
                if let Some(current) = this.try_at_mut(dimension) {
                    *current += *value
                } else {
                    this.insert(dimension, *value);
                }
            }
            this
        }
    }
    impl<T: Vectorize, U: Vectorize> std::ops::Add<GenericVectorRef<'_, U>> for GenericVector<T> {
        type Output = SparseVector;
        fn add(self, rhs: GenericVectorRef<U>) -> Self::Output {
            self.add_ref_ga(&rhs)
        }
    }
}

mod sub {
    use crate::operations::sub_ga::{SubGA, SubRefGA};

    use super::*;

    impl<T: Vectorize, U: Vectorize> SubGA<U> for GenericVector<T> {
        type Output = SparseVector;
        fn sub_ga(self, other: U) -> SparseVector {
            let mut this = SparseVector::from_vector(self.vector);
            let iter = other.into_enumerate();
            for (dimension, value) in iter {
                if let Some(current) = this.try_at_mut(dimension) {
                    *current -= value
                } else {
                    this.insert(dimension, -value);
                }
            }
            this
        }
    }
    impl<T: Vectorize, U: Vectorize> std::ops::Sub<U> for GenericVector<T> {
        type Output = SparseVector;
        fn sub(self, rhs: U) -> Self::Output {
            self.sub_ga(rhs)
        }
    }

    impl<T: Vectorize, U: Vectorize> SubRefGA<GenericVectorRef<'_, U>> for GenericVector<T> {
        type Output = SparseVector;
        fn sub_ref_ga(self, other: &GenericVectorRef<U>) -> SparseVector {
            let mut this = SparseVector::from_vector(self.vector);
            let iter = other.vector.enumerate();
            for (dimension, value) in iter {
                if let Some(current) = this.try_at_mut(dimension) {
                    *current -= *value
                } else {
                    this.insert(dimension, -*value);
                }
            }
            this
        }
    }
    impl<T: Vectorize, U: Vectorize> std::ops::Sub<GenericVectorRef<'_, U>> for GenericVector<T> {
        type Output = SparseVector;
        fn sub(self, rhs: GenericVectorRef<U>) -> Self::Output {
            self.sub_ref_ga(&rhs)
        }
    }
}

mod neg {
    use crate::operations::neg_ga::NegGA;

    use super::*;

    impl<T: Vectorize> NegGA for GenericVector<T> {
        fn neg_ga(&mut self) {
            for value in self.vector.iterate_values_mut() {
                *value = -(*value);
            }
        }
    }
    impl<T: Vectorize> std::ops::Neg for GenericVector<T> {
        type Output = T;
        fn neg(mut self) -> Self::Output {
            self.neg_ga();
            self.vector
        }
    }
}

mod mul {
    use crate::{algebras::scalar::Scalar, operations::mul_ga::MulGA};

    use super::*;

    impl<T: Vectorize> MulGA<Scalar> for GenericVector<T> {
        type Output = T;
        fn mul_ga(mut self, other: Scalar) -> T {
            for value in self.vector.iterate_values_mut() {
                *value *= other.internal();
            }
            self.vector
        }
    }
    impl<T: Vectorize> std::ops::Mul<Scalar> for GenericVector<T> {
        type Output = T;
        fn mul(self, rhs: Scalar) -> Self::Output {
            self.mul_ga(rhs)
        }
    }

    impl<T: Vectorize> MulGA<GenericVector<T>> for Scalar {
        type Output = T;
        fn mul_ga(self, mut other: GenericVector<T>) -> T {
            for value in other.vector.iterate_values_mut() {
                *value *= self.internal();
            }
            other.vector
        }
    }
    impl<T: Vectorize> std::ops::Mul<GenericVector<T>> for Scalar {
        type Output = T;
        fn mul(self, rhs: GenericVector<T>) -> Self::Output {
            self.mul_ga(rhs)
        }
    }
}

mod div {
    use crate::{algebras::scalar::Scalar, operations::div_ga::DivGA};

    use super::*;

    impl<T: Vectorize> DivGA<Scalar> for GenericVector<T> {
        type Output = T;
        fn div_ga(mut self, other: Scalar) -> T {
            for value in self.vector.iterate_values_mut() {
                *value /= other.internal();
            }
            self.vector
        }
    }
    impl<T: Vectorize> std::ops::Div<Scalar> for GenericVector<T> {
        type Output = T;
        fn div(self, rhs: Scalar) -> Self::Output {
            self.div_ga(rhs)
        }
    }
}

mod dot {
    use crate::{
        algebras::scalar::Scalar,
        operations::dot_ga::{DotGA, DotRefGA},
    };

    use super::*;

    impl<T: Vectorize, U: Vectorize> DotGA<U> for GenericVector<T> {
        type Output = Scalar;
        fn dot_ga(self, other: U) -> Self::Output {
            let mut scalar = 0.;

            for (dimension, value) in self.vector.into_enumerate() {
                if let Some(other_value) = other.try_at(dimension) {
                    scalar += value * *other_value
                }
            }

            Scalar::new(scalar)
        }
    }
    impl<T: Vectorize> GenericVector<T> {
        pub fn dot<U: Vectorize>(self, other: U) -> Scalar {
            self.dot_ga(other)
        }
    }

    impl<T: Vectorize, U: Vectorize> DotRefGA<GenericVectorRef<'_, U>> for GenericVector<T> {
        type Output = Scalar;
        fn dot_ref_ga(self, other: &GenericVectorRef<U>) -> Self::Output {
            let other = other.vector;

            let mut scalar = 0.;

            for (dimension, value) in self.vector.into_enumerate() {
                if let Some(other_value) = other.try_at(dimension) {
                    scalar += value * *other_value
                }
            }

            Scalar::new(scalar)
        }
    }
}
