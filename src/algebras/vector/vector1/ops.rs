use super::*;

use std::ops::*;

pub mod add {
    use super::*;

    impl Add for Vector1 {
        type Output = Self;

        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl AddAssign for Vector1 {
        fn add_assign(&mut self, rhs: Self) {
            *self.at_mut(Vector1Index::X) += rhs.at(Vector1Index::X);
        }
    }
}

pub mod sub {
    use super::*;

    impl Sub for Vector1 {
        type Output = Self;

        fn sub(mut self, rhs: Self) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl SubAssign for Vector1 {
        fn sub_assign(&mut self, rhs: Self) {
            *self.at_mut(Vector1Index::X) -= rhs.at(Vector1Index::X);
        }
    }
}

pub mod neg {
    use super::*;

    impl Neg for Vector1 {
        type Output = Self;

        fn neg(mut self) -> Self::Output {
            let x = self.at_mut(Vector1Index::X);
            *x = x.neg();
            self
        }
    }
}

pub mod mul {
    use crate::algebras::scalar::Scalar;

    use super::*;

    impl Mul<Scalar> for Vector1 {
        type Output = Vector1;

        fn mul(mut self, rhs: Scalar) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl MulAssign<Scalar> for Vector1 {
        fn mul_assign(&mut self, rhs: Scalar) {
            *self.at_mut(Vector1Index::X) *= *rhs;
        }
    }

    impl Mul<Vector1> for Scalar {
        type Output = Vector1;

        fn mul(self, mut rhs: Vector1) -> Self::Output {
            rhs *= self;
            rhs
        }
    }
}

pub mod abs {
    use crate::{algebras::scalar::Scalar, operations::Abs};

    use super::*;

    impl Abs for Vector1 {
        fn abs2(self) -> Scalar {
            Scalar::new(self.abs().powi(2))
        }

        fn abs(self) -> Scalar {
            Scalar::new(self.at(Vector1Index::X).abs())
        }
    }
}

pub mod inv {
    use crate::operations::{Inv, InvAssign};

    use super::*;

    impl Inv for Vector1 {
        type Output = Self;

        fn inv(mut self) -> Self {
            self.inv_assign();
            self
        }
    }

    impl InvAssign for Vector1 {
        fn inv_assign(&mut self) {
            let x = self.at_mut(Vector1Index::X);
            *x = x.recip();
        }
    }
}

pub mod div {
    use crate::algebras::scalar::Scalar;

    use super::*;

    impl Div<Scalar> for Vector1 {
        type Output = Vector1;

        fn div(mut self, rhs: Scalar) -> Self::Output {
            self /= rhs;
            self
        }
    }

    impl DivAssign<Scalar> for Vector1 {
        fn div_assign(&mut self, rhs: Scalar) {
            *self.at_mut(Vector1Index::X) /= *rhs;
        }
    }

    impl Div<Vector1> for Scalar {
        type Output = Vector1;

        fn div(self, mut rhs: Vector1) -> Self::Output {
            let x = rhs.at_mut(Vector1Index::X);
            *x = *self / *x;
            rhs
        }
    }
}

pub mod dot {
    use crate::{algebras::scalar::Scalar, operations::Dot};

    use super::*;

    impl Dot<Vector1> for Vector1 {
        type Output = Scalar;

        fn dot(self, rhs: Vector1) -> Self::Output {
            Scalar::new(*self.at(Vector1Index::X) * *rhs.at(Vector1Index::X))
        }
    }
}
