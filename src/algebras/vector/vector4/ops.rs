use crate::{
    algebras::scalar::Scalar,
    index_ga::{IndexGA, IndexGAMut},
    operations::{add_ga::AddGA, div_ga::DivGA, mul_ga::MulGA, sub_ga::SubGA},
};

use super::{Vector4, Vector4Index};

impl AddGA<Vector4> for Vector4 {
    type Output = Vector4;
    fn add_ga(mut self, other: Vector4) -> Self::Output {
        *self.at_mut(Vector4Index::X) += other.at(Vector4Index::X);
        *self.at_mut(Vector4Index::Y) += other.at(Vector4Index::Y);
        *self.at_mut(Vector4Index::Z) += other.at(Vector4Index::Z);
        *self.at_mut(Vector4Index::W) += other.at(Vector4Index::W);
        self
    }
}
impl std::ops::Add<Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, other: Vector4) -> Self::Output {
        self.add_ga(other)
    }
}

impl SubGA<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub_ga(mut self, other: Vector4) -> Self::Output {
        *self.at_mut(Vector4Index::X) -= other.at(Vector4Index::X);
        *self.at_mut(Vector4Index::Y) -= other.at(Vector4Index::Y);
        *self.at_mut(Vector4Index::Z) -= other.at(Vector4Index::Z);
        *self.at_mut(Vector4Index::W) -= other.at(Vector4Index::W);
        self
    }
}
impl std::ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, other: Vector4) -> Self::Output {
        self.sub_ga(other)
    }
}

impl MulGA<Scalar> for Vector4 {
    type Output = Vector4;
    fn mul_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector4Index::X) *= other.internal();
        *self.at_mut(Vector4Index::Y) *= other.internal();
        *self.at_mut(Vector4Index::Z) *= other.internal();
        *self.at_mut(Vector4Index::W) *= other.internal();
        self
    }
}
impl std::ops::Mul<Scalar> for Vector4 {
    type Output = Vector4;
    fn mul(self, other: Scalar) -> Self::Output {
        self.mul_ga(other)
    }
}

impl MulGA<Vector4> for Scalar {
    type Output = Vector4;
    fn mul_ga(self, mut other: Vector4) -> Self::Output {
        *other.at_mut(Vector4Index::X) *= self.internal();
        *other.at_mut(Vector4Index::Y) *= self.internal();
        *other.at_mut(Vector4Index::Z) *= self.internal();
        *other.at_mut(Vector4Index::W) *= self.internal();
        other
    }
}
impl std::ops::Mul<Vector4> for Scalar {
    type Output = Vector4;
    fn mul(self, other: Vector4) -> Self::Output {
        self.mul_ga(other)
    }
}

impl DivGA<Scalar> for Vector4 {
    type Output = Vector4;
    fn div_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector4Index::X) /= other.internal();
        *self.at_mut(Vector4Index::Y) /= other.internal();
        *self.at_mut(Vector4Index::Z) /= other.internal();
        *self.at_mut(Vector4Index::W) /= other.internal();
        self
    }
}
impl std::ops::Div<Scalar> for Vector4 {
    type Output = Vector4;
    fn div(self, other: Scalar) -> Self::Output {
        self.div_ga(other)
    }
}
