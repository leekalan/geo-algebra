use crate::{
    algebras::scalar::Scalar,
    index_ga::{IndexGA, IndexGAMut},
    operations::{
        add_ga::AddGA, div_ga::DivGA, dot_ga::DotGA, inv_ga::InvGA, mag_ga::MagGA, mul_ga::MulGA,
        neg_ga::NegGA, sub_ga::SubGA,
    },
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

impl NegGA for Vector4 {
    fn neg_ga(&mut self) {
        let x = self.at_mut(Vector4Index::X);
        *x = -*x;
        let y = self.at_mut(Vector4Index::Y);
        *y = -*y;
        let z = self.at_mut(Vector4Index::Z);
        *z = -*z;
        let w = self.at_mut(Vector4Index::W);
        *w = -*w;
    }
}
impl std::ops::Neg for Vector4 {
    type Output = Vector4;
    fn neg(mut self) -> Self::Output {
        self.neg_ga();
        self
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

impl InvGA for Vector4 {
    type Output = Vector4;
    fn inv_ga(mut self) -> Self {
        let x = self.at(Vector4Index::X);
        let y = self.at(Vector4Index::Y);
        let z = self.at(Vector4Index::Z);
        let w = self.at(Vector4Index::W);

        let denominator = x * x + y * y + z * z + w * w;

        *self.at_mut(Vector4Index::X) /= denominator;
        *self.at_mut(Vector4Index::Y) /= denominator;
        *self.at_mut(Vector4Index::Z) /= denominator;
        *self.at_mut(Vector4Index::W) /= denominator;

        self
    }
}
impl Vector4 {
    pub fn inv(self) -> Self {
        self.inv_ga()
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

impl DivGA<Vector4> for Scalar {
    type Output = Vector4;
    fn div_ga(self, mut other: Vector4) -> Self::Output {
        let x = other.at(Vector4Index::X);
        let y = other.at(Vector4Index::Y);
        let z = other.at(Vector4Index::Z);
        let w = other.at(Vector4Index::W);

        let denominator = x * x + y * y + z * z + w * w;

        let mult = self.internal() / denominator;

        *other.at_mut(Vector4Index::X) *= mult;
        *other.at_mut(Vector4Index::Y) *= mult;
        *other.at_mut(Vector4Index::Z) *= mult;
        *other.at_mut(Vector4Index::W) *= mult;

        other
    }
}
impl std::ops::Div<Vector4> for Scalar {
    type Output = Vector4;
    fn div(self, other: Vector4) -> Self::Output {
        self.div_ga(other)
    }
}

impl DotGA<Vector4> for Vector4 {
    type Output = Scalar;
    fn dot_ga(self, other: Vector4) -> Self::Output {
        let mut scalar;
        scalar = self.at(Vector4Index::X) * other.at(Vector4Index::X);
        scalar += self.at(Vector4Index::Y) * other.at(Vector4Index::Y);
        scalar += self.at(Vector4Index::Z) * other.at(Vector4Index::Z);
        scalar += self.at(Vector4Index::W) * other.at(Vector4Index::W);
        Scalar::new(scalar)
    }
}
impl Vector4 {
    pub fn dot(self, other: Vector4) -> Scalar {
        self.dot_ga(other)
    }
}

impl MagGA for Vector4 {
    fn mag2_ga(&self) -> Scalar {
        let mut scalar;
        scalar = self.at(Vector4Index::X).powi(2);
        scalar += self.at(Vector4Index::Y).powi(2);
        scalar += self.at(Vector4Index::Z).powi(2);
        scalar += self.at(Vector4Index::W).powi(2);
        Scalar::new(scalar)
    }
}
impl Vector4 {
    pub fn mag2(&self) -> Scalar {
        self.mag2_ga()
    }

    pub fn mag(&self) -> Scalar {
        self.mag_ga()
    }
}
