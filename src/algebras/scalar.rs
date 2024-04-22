use std::ops::{Deref, DerefMut};

use crate::index_ga::{IndexGA, IndexGAMut};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScalarIndex;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Scalar(f64);
impl Scalar {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}
impl Deref for Scalar {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Scalar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IndexGA<ScalarIndex> for Scalar {
    fn at(&self, _index: ScalarIndex) -> &f64 {
        &self.0
    }
}
impl IndexGAMut<ScalarIndex> for Scalar {
    fn at_mut(&mut self, _index: ScalarIndex) -> &mut f64 {
        &mut self.0
    }
}

mod ops {
    use crate::operations::{Abs, Inv};

    use super::*;

    use std::ops::*;

    impl Add for Scalar {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self::Output {
            *self += *rhs;
            self
        }
    }
    impl AddAssign for Scalar {
        fn add_assign(&mut self, rhs: Self) {
            **self += *rhs;
        }
    }

    impl Sub for Scalar {
        type Output = Self;
        fn sub(mut self, rhs: Self) -> Self::Output {
            *self -= *rhs;
            self
        }
    }
    impl SubAssign for Scalar {
        fn sub_assign(&mut self, rhs: Self) {
            **self -= *rhs;
        }
    }

    impl Neg for Scalar {
        type Output = Self;
        fn neg(mut self) -> Self::Output {
            *self = -*self;
            self
        }
    }

    impl Mul for Scalar {
        type Output = Self;
        fn mul(mut self, rhs: Self) -> Self::Output {
            *self *= *rhs;
            self
        }
    }
    impl MulAssign for Scalar {
        fn mul_assign(&mut self, rhs: Self) {
            **self *= *rhs;
        }
    }

    impl Div for Scalar {
        type Output = Self;
        fn div(mut self, rhs: Self) -> Self::Output {
            *self /= *rhs;
            self
        }
    }
    impl DivAssign for Scalar {
        fn div_assign(&mut self, rhs: Self) {
            **self /= *rhs;
        }
    }

    impl Inv for Scalar {
        type Output = Self;

        fn inv(mut self) -> Self::Output {
            *self = self.recip();
            self
        }
    }

    impl Abs for Scalar {
        fn abs2(self) -> Scalar {
            self * self
        }
        fn abs(self) -> Scalar {
            self
        }
    }
}
