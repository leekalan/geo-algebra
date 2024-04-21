use crate::{
    algebras::scalar::Scalar,
    index_ga::{IndexGA, IndexGAMut},
    operations::{add_ga::AddGA, div_ga::DivGA, inv_ga::InvGA, mul_ga::MulGA, sub_ga::SubGA},
};

use super::{Vector2, Vector2Index};

impl AddGA<Vector2> for Vector2 {
    type Output = Vector2;
    fn add_ga(mut self, other: Vector2) -> Self::Output {
        *self.at_mut(Vector2Index::X) += other.at(Vector2Index::X);
        *self.at_mut(Vector2Index::Y) += other.at(Vector2Index::Y);
        self
    }
}
impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Self::Output {
        self.add_ga(other)
    }
}

impl SubGA<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub_ga(mut self, other: Vector2) -> Self::Output {
        *self.at_mut(Vector2Index::X) -= other.at(Vector2Index::X);
        *self.at_mut(Vector2Index::Y) -= other.at(Vector2Index::Y);
        self
    }
}
impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, other: Vector2) -> Self::Output {
        self.sub_ga(other)
    }
}

impl MulGA<Scalar> for Vector2 {
    type Output = Vector2;
    fn mul_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector2Index::X) *= other.internal();
        *self.at_mut(Vector2Index::Y) *= other.internal();
        self
    }
}
impl std::ops::Mul<Scalar> for Vector2 {
    type Output = Vector2;
    fn mul(self, other: Scalar) -> Self::Output {
        self.mul_ga(other)
    }
}

impl MulGA<Vector2> for Scalar {
    type Output = Vector2;
    fn mul_ga(self, mut other: Vector2) -> Self::Output {
        *other.at_mut(Vector2Index::X) *= self.internal();
        *other.at_mut(Vector2Index::Y) *= self.internal();
        other
    }
}
impl std::ops::Mul<Vector2> for Scalar {
    type Output = Vector2;
    fn mul(self, other: Vector2) -> Self::Output {
        self.mul_ga(other)
    }
}

impl InvGA for Vector2 {
    type Output = Vector2;
    fn inv_ga(mut self) -> Self::Output {
        let x = self.at(Vector2Index::X);
        let y = self.at(Vector2Index::Y);

        let denominator = *x * *x + *y * *y;

        *self.at_mut(Vector2Index::X) /= denominator;
        *self.at_mut(Vector2Index::Y) /= -denominator;

        self
    }
}
impl Vector2 {
    pub fn inv(self) -> Self {
        self.inv_ga()
    }
}

impl DivGA<Scalar> for Vector2 {
    type Output = Vector2;
    fn div_ga(mut self, other: Scalar) -> Self::Output {
        *self.at_mut(Vector2Index::X) /= other.internal();
        *self.at_mut(Vector2Index::Y) /= other.internal();
        self
    }
}
impl std::ops::Div<Scalar> for Vector2 {
    type Output = Vector2;
    fn div(self, other: Scalar) -> Self::Output {
        self.div_ga(other)
    }
}

impl DivGA<Vector2> for Scalar {
    type Output = Vector2;
    fn div_ga(self, other: Vector2) -> Self::Output {
        self * other.inv()
    }
}
impl std::ops::Div<Vector2> for Scalar {
    type Output = Vector2;
    fn div(self, other: Vector2) -> Self::Output {
        self.div_ga(other)
    }
}
