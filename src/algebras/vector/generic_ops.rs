use super::*;

use std::ops::*;

pub mod add {
    use super::*;

    impl<T: Vectorize, U: Vectorize> Add<&U> for GenericVector<T> {
        type Output = SparseVector;

        fn add(self, rhs: &U) -> Self::Output {
            SparseVector::from_vector(self.vector) + rhs
        }
    }
    impl<T: Vectorize, U: Vectorize> Add<&U> for GenericVectorRef<'_, T> {
        type Output = SparseVector;

        fn add(self, rhs: &U) -> Self::Output {
            SparseVector::from_vector_ref(self.vector) + rhs
        }
    }
}

pub mod sub {
    use super::*;

    impl<T: Vectorize, U: Vectorize> Sub<&U> for GenericVector<T> {
        type Output = SparseVector;

        fn sub(self, rhs: &U) -> Self::Output {
            SparseVector::from_vector(self.vector) - rhs
        }
    }
    impl<T: Vectorize, U: Vectorize> Sub<&U> for GenericVectorRef<'_, T> {
        type Output = SparseVector;

        fn sub(self, rhs: &U) -> Self::Output {
            SparseVector::from_vector_ref(self.vector) - rhs
        }
    }
}

pub mod neg {
    use super::*;

    impl<T: Vectorize> Neg for GenericVector<T> {
        type Output = T;

        fn neg(mut self) -> Self::Output {
            for value in self.vector.iterate_values_mut() {
                *value = -*value;
            }
            self.vector
        }
    }
}

pub mod mul {
    use super::*;

    impl<T: Vectorize> Mul<Scalar> for GenericVector<T> {
        type Output = T;

        fn mul(mut self, rhs: Scalar) -> Self::Output {
            for value in self.vector.iterate_values_mut() {
                *value *= *rhs;
            }
            self.vector
        }
    }

    impl<T: Vectorize> Mul<GenericVector<T>> for Scalar {
        type Output = T;

        fn mul(self, mut rhs: GenericVector<T>) -> Self::Output {
            for value in rhs.vector.iterate_values_mut() {
                *value *= *self;
            }
            rhs.vector
        }
    }
}

pub mod abs {
    use crate::operations::Abs;

    use super::*;

    impl<T: Vectorize> Abs for GenericVectorRef<'_, T> {
        fn abs2(self) -> Scalar {
            Scalar::new(self.vector.iterate_values().map(|v| v.powi(2)).sum())
        }
    }
}

pub mod inv {
    use crate::operations::{Abs, Inv};

    use super::*;

    impl<T: Vectorize> Inv for GenericVector<T> {
        type Output = T;

        fn inv(self) -> Self::Output {
            let denominator = self.to_ref().abs2();
            self / denominator
        }
    }
}

pub mod div {
    use crate::operations::{Abs, Inv};

    use super::*;

    impl<T: Vectorize> Div<Scalar> for GenericVector<T> {
        type Output = T;

        #[allow(clippy::suspicious_arithmetic_impl)]
        fn div(self, rhs: Scalar) -> Self::Output {
            let mult = rhs.inv();
            self * mult
        }
    }

    impl<T: Vectorize> Div<GenericVector<T>> for Scalar {
        type Output = T;

        fn div(self, rhs: GenericVector<T>) -> Self::Output {
            let mult = self / rhs.to_ref().abs2();
            rhs * mult
        }
    }
}

pub mod dot {
    use crate::operations::Dot;

    use super::*;

    impl<T: Vectorize, U: Vectorize> Dot<&U> for GenericVectorRef<'_, T> {
        type Output = Scalar;

        fn dot(self, rhs: &U) -> Self::Output {
            let mut scalar = 0.;

            if self.vector.range() < rhs.range() {
                for (dimension, value) in self.vector.enumerate() {
                    if let Some(other) = rhs.try_at(dimension) {
                        scalar += value * other;
                    }
                }
            } else {
                for (dimension, value) in rhs.enumerate() {
                    if let Some(other) = self.vector.try_at(dimension) {
                        scalar += value * other;
                    }
                }
            }

            Scalar::new(scalar)
        }
    }
}
