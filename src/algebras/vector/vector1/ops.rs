use crate::{
    algebras::scalar::Scalar,
    index_ga::{IndexGA, IndexGAMut},
    operations::{add_ga::AddGA, div_ga::DivGA, inv_ga::InvGA, mul_ga::MulGA, sub_ga::SubGA},
};

use super::{Vector1, Vector1Index};

impl AddGA<Vector1> for Vector1 {
    type Output = Vector1;
    fn add_ga(mut self, other: Vector1) -> Self::Output {
        *self.at_mut(Vector1Index::X) += other.at(Vector1Index::X);
        self
    }
}
impl std::ops::Add<Vector1> for Vector1 {
    type Output = Vector1;
    fn add(self, other: Vector1) -> Self::Output {
        self.add_ga(other)
    }
}

impl SubGA<Vector1> for Vector1 {
    type Output = Vector1;
    fn sub_ga(mut self, other: Vector1) -> Self::Output {
        *self.at_mut(Vector1Index::X) -= other.at(Vector1Index::X);
        self
    }
}
impl std::ops::Sub<Vector1> for Vector1 {
    type Output = Vector1;
    fn sub(self, other: Vector1) -> Self::Output {
        self.sub_ga(other)
    }
}

impl MulGA<Scalar> for Vector1 {
    type Output = Vector1;
    fn mul_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector1Index::X) *= other.internal();
        self
    }
}
impl std::ops::Mul<Scalar> for Vector1 {
    type Output = Vector1;
    fn mul(self, other: Scalar) -> Self::Output {
        self.mul_ga(other)
    }
}

impl MulGA<Vector1> for Scalar {
    type Output = Vector1;
    fn mul_ga(self, mut other: Vector1) -> Self::Output {
        *other.at_mut(Vector1Index::X) *= self.internal();
        other
    }
}
impl std::ops::Mul<Vector1> for Scalar {
    type Output = Vector1;
    fn mul(self, other: Vector1) -> Self::Output {
        self.mul_ga(other)
    }
}

impl InvGA for Vector1 {
    type Output = Vector1;
    fn inv_ga(mut self) -> Self::Output {
        let this = self.at_mut(Vector1Index::X);
        *this = this.recip();
        self
    }
}
impl Vector1 {
    pub fn inv(self) -> Self {
        self.inv_ga()
    }
}

impl DivGA<Scalar> for Vector1 {
    type Output = Vector1;
    fn div_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector1Index::X) /= other.internal();
        self
    }
}
impl std::ops::Div<Scalar> for Vector1 {
    type Output = Vector1;
    fn div(self, other: Scalar) -> Self::Output {
        self.div_ga(other)
    }
}

impl DivGA<Vector1> for Scalar {
    type Output = Vector1;
    fn div_ga(self, other: Vector1) -> Self::Output {
        self * other.inv()
    }
}
impl std::ops::Div<Vector1> for Scalar {
    type Output = Vector1;
    fn div(self, other: Vector1) -> Self::Output {
        self.div_ga(other)
    }
}
