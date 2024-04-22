use super::*;

use std::ops::*;

pub mod add {
    use super::*;

    impl Add for Vector4 {
        type Output = Self;

        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl AddAssign for Vector4 {
        fn add_assign(&mut self, rhs: Self) {
            *self.at_mut(Vector4Index::X) += rhs.at(Vector4Index::X);
            *self.at_mut(Vector4Index::Y) += rhs.at(Vector4Index::Y);
            *self.at_mut(Vector4Index::Z) += rhs.at(Vector4Index::Z);
            *self.at_mut(Vector4Index::W) += rhs.at(Vector4Index::W);
        }
    }
}

pub mod sub {
    use super::*;

    impl Sub for Vector4 {
        type Output = Self;

        fn sub(mut self, rhs: Self) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl SubAssign for Vector4 {
        fn sub_assign(&mut self, rhs: Self) {
            *self.at_mut(Vector4Index::X) -= rhs.at(Vector4Index::X);
            *self.at_mut(Vector4Index::Y) -= rhs.at(Vector4Index::Y);
            *self.at_mut(Vector4Index::Z) -= rhs.at(Vector4Index::Z);
            *self.at_mut(Vector4Index::W) -= rhs.at(Vector4Index::W);
        }
    }
}

pub mod neg {
    use super::*;

    impl Neg for Vector4 {
        type Output = Self;

        fn neg(mut self) -> Self::Output {
            let x = self.at_mut(Vector4Index::X);
            *x = x.neg();
            let y = self.at_mut(Vector4Index::Y);
            *y = y.neg();
            let z = self.at_mut(Vector4Index::Z);
            *z = z.neg();
            let w = self.at_mut(Vector4Index::W);
            *w = w.neg();
            self
        }
    }
}

pub mod mul {
    use crate::algebras::scalar::Scalar;

    use super::*;

    impl Mul<Scalar> for Vector4 {
        type Output = Vector4;

        fn mul(mut self, rhs: Scalar) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl MulAssign<Scalar> for Vector4 {
        fn mul_assign(&mut self, rhs: Scalar) {
            *self.at_mut(Vector4Index::X) *= *rhs;
            *self.at_mut(Vector4Index::Y) *= *rhs;
            *self.at_mut(Vector4Index::Z) *= *rhs;
            *self.at_mut(Vector4Index::W) *= *rhs;
        }
    }

    impl Mul<Vector4> for Scalar {
        type Output = Vector4;

        fn mul(self, mut rhs: Vector4) -> Self::Output {
            rhs *= self;
            rhs
        }
    }
}

pub mod abs {
    use crate::{algebras::scalar::Scalar, operations::Abs};

    use super::*;

    impl Abs for Vector4 {
        fn abs2(self) -> Scalar {
            let mut scalar;
            scalar = self.at(Vector4Index::X).powi(2);
            scalar += self.at(Vector4Index::Y).powi(2);
            scalar += self.at(Vector4Index::Z).powi(2);
            scalar += self.at(Vector4Index::W).powi(2);
            Scalar::new(scalar)
        }
    }
}

pub mod inv {
    use crate::operations::{Abs, Inv, InvAssign};

    use super::*;

    impl Inv for Vector4 {
        type Output = Self;

        fn inv(mut self) -> Self {
            self.inv_assign();
            self
        }
    }

    impl InvAssign for Vector4 {
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

    impl Div<Scalar> for Vector4 {
        type Output = Vector4;

        fn div(mut self, rhs: Scalar) -> Self::Output {
            self /= rhs;
            self
        }
    }

    #[allow(clippy::suspicious_op_assign_impl)]
    impl DivAssign<Scalar> for Vector4 {
        fn div_assign(&mut self, rhs: Scalar) {
            let inv = rhs.inv();
            *self *= inv;
        }
    }

    impl Div<Vector4> for Scalar {
        type Output = Vector4;

        fn div(self, rhs: Vector4) -> Self::Output {
            let mult = self / rhs.abs2();
            rhs * mult
        }
    }
}

pub mod dot {
    use crate::{algebras::scalar::Scalar, operations::Dot};

    use super::*;

    impl Dot<Vector4> for Vector4 {
        type Output = Scalar;

        fn dot(self, rhs: Vector4) -> Self::Output {
            let mut scalar;
            scalar = *self.at(Vector4Index::X) * *rhs.at(Vector4Index::X);
            scalar += *self.at(Vector4Index::Y) * *rhs.at(Vector4Index::Y);
            scalar += *self.at(Vector4Index::Z) * *rhs.at(Vector4Index::Z);
            scalar += *self.at(Vector4Index::W) * *rhs.at(Vector4Index::W);
            Scalar::new(scalar)
        }
    }
}
