use super::*;

use std::ops::*;

pub mod add {
    use super::*;

    impl<U: Vectorize> Add<&U> for SparseVector {
        type Output = Self;

        fn add(mut self, rhs: &U) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl<U: Vectorize> AddAssign<&U> for SparseVector {
        fn add_assign(&mut self, rhs: &U) {
            for (dimension, value) in rhs.enumerate() {
                if let Some(current) = self.try_at_mut(dimension) {
                    *current += value;
                } else {
                    self.insert(dimension, *value);
                }
            }
        }
    }
}

pub mod sub {
    use super::*;

    impl<U: Vectorize> Sub<&U> for SparseVector {
        type Output = Self;

        fn sub(mut self, rhs: &U) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl<U: Vectorize> SubAssign<&U> for SparseVector {
        fn sub_assign(&mut self, rhs: &U) {
            for (dimension, value) in rhs.enumerate() {
                if let Some(current) = self.try_at_mut(dimension) {
                    *current -= value;
                } else {
                    self.insert(dimension, -*value);
                }
            }
        }
    }
}

pub mod neg {
    use super::*;

    impl Neg for SparseVector {
        type Output = Self;

        fn neg(mut self) -> Self::Output {
            for value in self.iterate_values_mut() {
                *value = -*value;
            }
            self
        }
    }
}

pub mod mul {
    use crate::algebras::scalar::Scalar;

    use super::*;

    impl Mul<Scalar> for SparseVector {
        type Output = Self;

        fn mul(mut self, rhs: Scalar) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl MulAssign<Scalar> for SparseVector {
        fn mul_assign(&mut self, rhs: Scalar) {
            for value in self.iterate_values_mut() {
                *value *= *rhs;
            }
        }
    }

    impl Mul<SparseVector> for Scalar {
        type Output = SparseVector;

        fn mul(self, mut rhs: SparseVector) -> Self::Output {
            rhs *= self;
            rhs
        }
    }
}

pub mod abs {
    use crate::{algebras::scalar::Scalar, operations::Abs};

    use super::*;

    impl Abs for &SparseVector {
        fn abs2(self) -> Scalar {
            Scalar::new(self.iterate_values().map(|v| v.powi(2)).sum())
        }
    }
}

pub mod inv {
    use crate::operations::{Abs, Inv};

    use super::*;

    impl Inv for SparseVector {
        type Output = Self;

        fn inv(self) -> Self::Output {
            let denominator = self.abs2();
            self / denominator
        }
    }
}

pub mod div {
    use crate::{
        algebras::scalar::Scalar,
        operations::{Abs, Inv},
    };

    use super::*;

    impl Div<Scalar> for SparseVector {
        type Output = Self;

        fn div(mut self, rhs: Scalar) -> Self::Output {
            self /= rhs;
            self
        }
    }

    impl DivAssign<Scalar> for SparseVector {
        #[allow(clippy::suspicious_op_assign_impl)]
        fn div_assign(&mut self, rhs: Scalar) {
            let mult = rhs.inv();
            *self *= mult;
        }
    }

    impl Div<SparseVector> for Scalar {
        type Output = SparseVector;

        fn div(self, rhs: SparseVector) -> Self::Output {
            let mult = self / rhs.abs2();
            rhs * mult
        }
    }
}

pub mod dot {
    use crate::{algebras::scalar::Scalar, operations::Dot};

    use super::*;

    impl<U: Vectorize> Dot<&U> for &SparseVector {
        type Output = Scalar;

        fn dot(self, rhs: &U) -> Self::Output {
            let mut scalar = 0.;

            if self.range() < rhs.range() {
                for (dimension, value) in self.enumerate() {
                    if let Some(other) = rhs.try_at(dimension) {
                        scalar += value * other;
                    }
                }
            } else {
                for (dimension, value) in rhs.enumerate() {
                    if let Some(other) = self.try_at(dimension) {
                        scalar += value * other;
                    }
                }
            }

            Scalar::new(scalar)
        }
    }
}
