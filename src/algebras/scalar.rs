use crate::{index_ga::{IndexGA, IndexGAMut}, operations::{add_ga::AddGA, div_ga::DivGA, mul_ga::MulGA, neg_ga::NegGA, sub_ga::SubGA}};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScalarIndex;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Scalar(f64);
impl Scalar {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
    pub fn internal(&self) -> f64 {
        self.0
    }
}
impl AsRef<f64> for Scalar {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}
impl AsMut<f64> for Scalar {
    fn as_mut(&mut self) -> &mut f64 {
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

impl AddGA<Scalar> for Scalar {
    type Output = Scalar;
    fn add_ga(self, other: Scalar) -> Scalar {
        Scalar(self.0 + other.0)
    }
}
impl std::ops::Add<Scalar> for Scalar {
    type Output = Scalar;
    fn add(self, rhs: Scalar) -> Self::Output {
        self.add_ga(rhs)
    }
}
impl std::ops::AddAssign<Scalar> for Scalar {
    fn add_assign(&mut self, rhs: Scalar) {
        self.0 += rhs.0
    }
}

impl SubGA<Scalar> for Scalar {
    type Output = Scalar;
    fn sub_ga(self, other: Scalar) -> Scalar {
        Scalar(self.0 - other.0)
    }
}
impl std::ops::Sub<Scalar> for Scalar {
    type Output = Scalar;
    fn sub(self, rhs: Scalar) -> Self::Output {
        self.sub_ga(rhs)
    }
}
impl std::ops::SubAssign<Scalar> for Scalar {
    fn sub_assign(&mut self, rhs: Scalar) {
        self.0 -= rhs.0
    }
}

impl MulGA<Scalar> for Scalar {
    type Output = Scalar;
    fn mul_ga(self, other: Scalar) -> Scalar {
        Scalar(self.0 * other.0)
    }
}
impl std::ops::Mul<Scalar> for Scalar {
    type Output = Scalar;
    fn mul(self, rhs: Scalar) -> Self::Output {
        self.mul_ga(rhs)
    }
}
impl std::ops::MulAssign<Scalar> for Scalar {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.0 *= rhs.0
    }
}

impl DivGA<Scalar> for Scalar {
    type Output = Scalar;
    fn div_ga(self, other: Scalar) -> Scalar {
        Scalar(self.0 / other.0)
    }
}
impl std::ops::Div<Scalar> for Scalar {
    type Output = Scalar;
    fn div(self, rhs: Scalar) -> Self::Output {
        self.div_ga(rhs)
    }
}
impl std::ops::DivAssign<Scalar> for Scalar {
    fn div_assign(&mut self, rhs: Scalar) {
        self.0 /= rhs.0
    }
}

impl NegGA for Scalar {
    fn neg_ga(&mut self) {
        self.0 = -self.0
    }
}
impl std::ops::Neg for Scalar {
    type Output = Scalar;
    fn neg(mut self) -> Self::Output {
        self.neg_ga();
        self
    }
}