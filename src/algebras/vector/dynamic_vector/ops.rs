use super::*;

use std::ops::*;

pub mod add {
    use super::*;

    impl Add<&Self> for DynamicVector {
        type Output = Self;

        fn add(mut self, rhs: &Self) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl AddAssign<&Self> for DynamicVector {
        fn add_assign(&mut self, rhs: &Self) {
            let lhs_range = self.range();
            let rhs_range = rhs.range();
            if lhs_range > rhs_range {
                for (lhs, rhs) in self.iterate_values_mut().zip(rhs.iterate_values()) {
                    *lhs += rhs;
                }
            } else {
                self.dimensions.reserve(rhs_range - lhs_range);
                let rhs_end = unsafe { rhs.dimensions.get_unchecked(lhs_range..) };
                for (lhs, rhs) in self.iterate_values_mut().zip(rhs.iterate_values()) {
                    *lhs += rhs;
                }
                self.dimensions.extend(rhs_end.iter());
            }
        }
    }
}

pub mod sub {
    use super::*;

    impl Sub<&Self> for DynamicVector {
        type Output = Self;

        fn sub(mut self, rhs: &Self) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl SubAssign<&Self> for DynamicVector {
        fn sub_assign(&mut self, rhs: &Self) {
            let lhs_range = self.range();
            let rhs_range = rhs.range();
            if lhs_range > rhs_range {
                for (lhs, rhs) in self.iterate_values_mut().zip(rhs.iterate_values()) {
                    *lhs -= rhs;
                }
            } else {
                self.dimensions.reserve(rhs_range - lhs_range);
                let rhs_end = unsafe { rhs.dimensions.get_unchecked(lhs_range..) };
                for (lhs, rhs) in self.iterate_values_mut().zip(rhs.iterate_values()) {
                    *lhs -= rhs;
                }
                self.dimensions.extend(rhs_end.iter().map(|v| -v));
            }
        }
    }
}

pub mod neg {
    use super::*;

    impl Neg for DynamicVector {
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

    impl Mul<Scalar> for DynamicVector {
        type Output = Self;

        fn mul(mut self, rhs: Scalar) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl MulAssign<Scalar> for DynamicVector {
        fn mul_assign(&mut self, rhs: Scalar) {
            for value in self.iterate_values_mut() {
                *value *= *rhs;
            }
        }
    }

    impl Mul<DynamicVector> for Scalar {
        type Output = DynamicVector;

        fn mul(self, mut rhs: DynamicVector) -> Self::Output {
            rhs *= self;
            rhs
        }
    }
}

pub mod abs {
    use crate::{algebras::scalar::Scalar, operations::Abs};

    use super::*;

    impl Abs for &DynamicVector {
        fn abs2(self) -> Scalar {
            Scalar::new(self.iterate_values().map(|v| v.powi(2)).sum())
        }
    }
}

pub mod inv {
    use crate::operations::{Abs, Inv, InvAssign};

    use super::*;

    impl Inv for DynamicVector {
        type Output = Self;

        fn inv(mut self) -> Self {
            self.inv_assign();
            self
        }
    }

    impl InvAssign for DynamicVector {
        fn inv_assign(&mut self) {
            let denominator = self.abs2();
            *self /= denominator;
        }
    }
}

pub mod div {
    use crate::{
        algebras::scalar::Scalar,
        operations::{Abs, Inv},
    };

    use super::*;

    impl Div<Scalar> for DynamicVector {
        type Output = Self;

        fn div(mut self, rhs: Scalar) -> Self::Output {
            self /= rhs;
            self
        }
    }

    #[allow(clippy::suspicious_op_assign_impl)]
    impl DivAssign<Scalar> for DynamicVector {
        fn div_assign(&mut self, rhs: Scalar) {
            let mult = rhs.inv();
            *self *= mult;
        }
    }

    impl Div<DynamicVector> for Scalar {
        type Output = DynamicVector;

        fn div(self, rhs: DynamicVector) -> Self::Output {
            let mult = self / rhs.abs2();
            rhs * mult
        }
    }
}

pub mod dot {
    use crate::{algebras::scalar::Scalar, operations::Dot};

    use super::*;

    impl Dot<&Self> for &DynamicVector {
        type Output = Scalar;

        fn dot(self, other: &Self) -> Self::Output {
            Scalar::new(
                self.iterate_values()
                    .zip(other.iterate_values())
                    .map(|(a, b)| a * b)
                    .sum(),
            )
        }
    }
}
