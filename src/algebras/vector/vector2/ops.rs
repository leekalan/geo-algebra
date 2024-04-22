use super::*;

use std::ops::*;

pub mod add {
    use super::*;

    impl Add for Vector2 {
        type Output = Self;

        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl AddAssign for Vector2 {
        fn add_assign(&mut self, rhs: Self) {
            *self.at_mut(Vector2Index::X) += rhs.at(Vector2Index::X);
            *self.at_mut(Vector2Index::Y) += rhs.at(Vector2Index::Y);
        }
    }
}

pub mod sub {
    use super::*;

    impl Sub for Vector2 {
        type Output = Self;

        fn sub(mut self, rhs: Self) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl SubAssign for Vector2 {
        fn sub_assign(&mut self, rhs: Self) {
            *self.at_mut(Vector2Index::X) -= rhs.at(Vector2Index::X);
            *self.at_mut(Vector2Index::Y) -= rhs.at(Vector2Index::Y);
        }
    }
}

pub mod neg {
    use super::*;

    impl Neg for Vector2 {
        type Output = Self;

        fn neg(mut self) -> Self::Output {
            let x = self.at_mut(Vector2Index::X);
            *x = x.neg();
            let y = self.at_mut(Vector2Index::Y);
            *y = y.neg();
            self
        }
    }
}

pub mod mul {
    use crate::algebras::scalar::Scalar;

    use super::*;

    impl Mul<Scalar> for Vector2 {
        type Output = Vector2;

        fn mul(mut self, rhs: Scalar) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl MulAssign<Scalar> for Vector2 {
        fn mul_assign(&mut self, rhs: Scalar) {
            *self.at_mut(Vector2Index::X) *= *rhs;
            *self.at_mut(Vector2Index::Y) *= *rhs;
        }
    }

    impl Mul<Vector2> for Scalar {
        type Output = Vector2;

        fn mul(self, mut rhs: Vector2) -> Self::Output {
            rhs *= self;
            rhs
        }
    }
}

pub mod abs {
    use crate::{algebras::scalar::Scalar, operations::Abs};

    use super::*;

    impl Abs for Vector2 {
        fn abs2(self) -> Scalar {
            let mut scalar;
            scalar = self.at(Vector2Index::X).powi(2);
            scalar += self.at(Vector2Index::Y).powi(2);
            Scalar::new(scalar)
        }
    }
}

pub mod inv {
    use crate::operations::{Abs, Inv, InvAssign};

    use super::*;

    impl Inv for Vector2 {
        type Output = Self;

        fn inv(mut self) -> Self {
            self.inv_assign();
            self
        }
    }

    impl InvAssign for Vector2 {
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

    impl Div<Scalar> for Vector2 {
        type Output = Vector2;

        fn div(mut self, rhs: Scalar) -> Self::Output {
            self /= rhs;
            self
        }
    }

    #[allow(clippy::suspicious_op_assign_impl)]
    impl DivAssign<Scalar> for Vector2 {
        fn div_assign(&mut self, rhs: Scalar) {
            let inv = rhs.inv();
            *self *= inv;
        }
    }

    impl Div<Vector2> for Scalar {
        type Output = Vector2;

        fn div(self, rhs: Vector2) -> Self::Output {
            let mult = self / rhs.abs2();
            rhs * mult
        }
    }
}

pub mod dot {
    use crate::{algebras::scalar::Scalar, operations::Dot};

    use super::*;

    impl Dot<Vector2> for Vector2 {
        type Output = Scalar;

        fn dot(self, rhs: Vector2) -> Self::Output {
            let mut scalar;
            scalar = *self.at(Vector2Index::X) * *rhs.at(Vector2Index::X);
            scalar += *self.at(Vector2Index::Y) * *rhs.at(Vector2Index::Y);
            Scalar::new(scalar)
        }
    }
}
