use crate::{
    algebras::scalar::Scalar,
    index_ga::{IndexGA, IndexGAMut},
    operations::{
        add_ga::AddGA,
        div_ga::DivGA,
        dot_ga::{DotGA, DotRefGA},
        inv_ga::InvGA,
        mag_ga::MagGA,
        mul_ga::MulGA,
        neg_ga::NegGA,
        sub_ga::SubGA,
    },
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

impl NegGA for Vector2 {
    fn neg_ga(&mut self) {
        let x = self.at_mut(Vector2Index::X);
        *x = -*x;
        let y = self.at_mut(Vector2Index::Y);
        *y = -*y;
    }
}
impl std::ops::Neg for Vector2 {
    type Output = Vector2;
    fn neg(mut self) -> Self::Output {
        self.neg_ga();
        self
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

        let denominator = x * x + y * y;

        *self.at_mut(Vector2Index::X) /= denominator;
        *self.at_mut(Vector2Index::Y) /= denominator;

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
    fn div_ga(self, mut other: Vector2) -> Self::Output {
        let x = other.at(Vector2Index::X);
        let y = other.at(Vector2Index::Y);

        let denominator = x * x + y * y;

        let mult = self.internal() / denominator;

        *other.at_mut(Vector2Index::X) *= mult;
        *other.at_mut(Vector2Index::Y) *= mult;

        other
    }
}
impl std::ops::Div<Vector2> for Scalar {
    type Output = Vector2;
    fn div(self, other: Vector2) -> Self::Output {
        self.div_ga(other)
    }
}

impl DotRefGA<Vector2> for Vector2 {
    type Output = Scalar;
    fn dot_ref_ga(self, other: &Vector2) -> Self::Output {
        let mut scalar;
        scalar = self.at(Vector2Index::X) * other.at(Vector2Index::X);
        scalar += self.at(Vector2Index::Y) * other.at(Vector2Index::Y);
        Scalar::new(scalar)
    }
}
impl Vector2 {
    pub fn dot(self, other: &Vector2) -> Scalar {
        self.dot_ga(other)
    }
}

impl MagGA for Vector2 {
    fn mag2_ga(&self) -> Scalar {
        let mut scalar;
        scalar = self.at(Vector2Index::X).powi(2);
        scalar += self.at(Vector2Index::Y).powi(2);
        Scalar::new(scalar)
    }
}
impl Vector2 {
    pub fn mag2(&self) -> Scalar {
        self.mag2_ga()
    }

    pub fn mag(&self) -> Scalar {
        self.mag_ga()
    }
}
