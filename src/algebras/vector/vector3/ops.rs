use crate::{
    algebras::scalar::Scalar,
    index_ga::{IndexGA, IndexGAMut},
    operations::{add_ga::AddGA, div_ga::DivGA, mul_ga::MulGA, sub_ga::SubGA},
};

use super::{Vector3, Vector3Index};

impl AddGA<Vector3> for Vector3 {
    type Output = Vector3;
    fn add_ga(mut self, other: Vector3) -> Self::Output {
        *self.at_mut(Vector3Index::X) += other.at(Vector3Index::X);
        *self.at_mut(Vector3Index::Y) += other.at(Vector3Index::Y);
        *self.at_mut(Vector3Index::Z) += other.at(Vector3Index::Z);
        self
    }
}
impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Self::Output {
        self.add_ga(other)
    }
}

impl SubGA<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub_ga(mut self, other: Vector3) -> Self::Output {
        *self.at_mut(Vector3Index::X) -= other.at(Vector3Index::X);
        *self.at_mut(Vector3Index::Y) -= other.at(Vector3Index::Y);
        *self.at_mut(Vector3Index::Z) -= other.at(Vector3Index::Z);
        self
    }
}
impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Self::Output {
        self.sub_ga(other)
    }
}

impl MulGA<Scalar> for Vector3 {
    type Output = Vector3;
    fn mul_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector3Index::X) *= other.internal();
        *self.at_mut(Vector3Index::Y) *= other.internal();
        *self.at_mut(Vector3Index::Z) *= other.internal();
        self
    }
}
impl std::ops::Mul<Scalar> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Scalar) -> Self::Output {
        self.mul_ga(other)
    }
}

impl MulGA<Vector3> for Scalar {
    type Output = Vector3;
    fn mul_ga(self, mut other: Vector3) -> Self::Output {
        *other.at_mut(Vector3Index::X) *= self.internal();
        *other.at_mut(Vector3Index::Y) *= self.internal();
        *other.at_mut(Vector3Index::Z) *= self.internal();
        other
    }
}
impl std::ops::Mul<Vector3> for Scalar {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Self::Output {
        self.mul_ga(other)
    }
}

impl DivGA<Scalar> for Vector3 {
    type Output = Vector3;
    fn div_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector3Index::X) /= other.internal();
        *self.at_mut(Vector3Index::Y) /= other.internal();
        *self.at_mut(Vector3Index::Z) /= other.internal();
        self
    }
}
impl std::ops::Div<Scalar> for Vector3 {
    type Output = Vector3;
    fn div(self, other: Scalar) -> Self::Output {
        self.div_ga(other)
    }
}
