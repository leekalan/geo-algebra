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

impl NegGA for Vector3 {
    fn neg_ga(&mut self) {
        let x = self.at_mut(Vector3Index::X);
        *x = -*x;
        let y = self.at_mut(Vector3Index::Y);
        *y = -*y;
        let z = self.at_mut(Vector3Index::Z);
        *z = -*z;
    }
}
impl std::ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(mut self) -> Self::Output {
        self.neg_ga();
        self
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

impl InvGA for Vector3 {
    type Output = Vector3;
    fn inv_ga(mut self) -> Self::Output {
        let x = self.at(Vector3Index::X);
        let y = self.at(Vector3Index::Y);
        let z = self.at(Vector3Index::Z);

        let denominator = x * x + y * y + z * z;

        *self.at_mut(Vector3Index::X) /= denominator;
        *self.at_mut(Vector3Index::Y) /= denominator;
        *self.at_mut(Vector3Index::Z) /= denominator;

        self
    }
}
impl Vector3 {
    pub fn inv(self) -> Self {
        self.inv_ga()
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

impl DivGA<Vector3> for Scalar {
    type Output = Vector3;
    fn div_ga(self, mut other: Vector3) -> Self::Output {
        let x = other.at(Vector3Index::X);
        let y = other.at(Vector3Index::Y);
        let z = other.at(Vector3Index::Z);

        let denominator = x * x + y * y + z * z;

        let mult = self.internal() / denominator;

        *other.at_mut(Vector3Index::X) *= mult;
        *other.at_mut(Vector3Index::Y) *= mult;
        *other.at_mut(Vector3Index::Z) *= mult;

        other
    }
}
impl std::ops::Div<Vector3> for Scalar {
    type Output = Vector3;
    fn div(self, other: Vector3) -> Self::Output {
        self.div_ga(other)
    }
}

impl DotRefGA<Vector3> for Vector3 {
    type Output = Scalar;
    fn dot_ref_ga(self, other: &Vector3) -> Self::Output {
        let mut scalar;
        scalar = self.at(Vector3Index::X) * other.at(Vector3Index::X);
        scalar += self.at(Vector3Index::Y) * other.at(Vector3Index::Y);
        scalar += self.at(Vector3Index::Z) * other.at(Vector3Index::Z);
        Scalar::new(scalar)
    }
}
impl Vector3 {
    pub fn dot(self, other: &Vector3) -> Scalar {
        self.dot_ga(other)
    }
}

impl MagGA for Vector3 {
    fn mag2_ga(&self) -> Scalar {
        let mut scalar;
        scalar = self.at(Vector3Index::X).powi(2);
        scalar += self.at(Vector3Index::Y).powi(2);
        scalar += self.at(Vector3Index::Z).powi(2);
        Scalar::new(scalar)
    }
}
impl Vector3 {
    pub fn mag2(&self) -> Scalar {
        self.mag2_ga()
    }

    pub fn mag(&self) -> Scalar {
        self.mag_ga()
    }
}
