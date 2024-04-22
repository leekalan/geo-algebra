use std::{
    mem::take,
    ops::{Mul, Neg},
};

use crate::{
    algebras::{scalar::Scalar, vector::Vectorize},
    operations::{
        add_ga::AddRefGA, div_ga::DivGA, dot_ga::DotRefGA, mag_ga::MagGA, mul_ga::MulGA,
        neg_ga::NegGA, sub_ga::SubRefGA,
    },
};

use super::SparseVector;

impl AddRefGA<SparseVector> for SparseVector {
    type Output = SparseVector;
    fn add_ref_ga(self, other: &SparseVector) -> Self::Output {
        self.generic_vector() + other.generic_vector_ref()
    }
}
impl std::ops::Add<&SparseVector> for SparseVector {
    type Output = SparseVector;
    fn add(self, other: &SparseVector) -> Self::Output {
        self.add_ref_ga(other)
    }
}

impl SubRefGA<SparseVector> for SparseVector {
    type Output = SparseVector;
    fn sub_ref_ga(self, other: &SparseVector) -> Self::Output {
        self.generic_vector() - other.generic_vector_ref()
    }
}
impl std::ops::Sub<&SparseVector> for SparseVector {
    type Output = SparseVector;
    fn sub(self, other: &SparseVector) -> Self::Output {
        self.sub_ref_ga(other)
    }
}

impl NegGA for SparseVector {
    fn neg_ga(&mut self) {
        let this = take(self);
        *self = take(&mut this.generic_vector().neg());
    }
}
impl Neg for SparseVector {
    type Output = SparseVector;
    fn neg(mut self) -> Self::Output {
        self.neg_ga();
        self
    }
}

impl MulGA<Scalar> for SparseVector {
    type Output = SparseVector;
    fn mul_ga(self, other: Scalar) -> Self::Output {
        self.generic_vector() * other
    }
}
impl Mul<Scalar> for SparseVector {
    type Output = SparseVector;
    fn mul(self, rhs: Scalar) -> Self::Output {
        self.mul_ga(rhs)
    }
}

impl MulGA<SparseVector> for Scalar {
    type Output = SparseVector;
    fn mul_ga(self, other: SparseVector) -> Self::Output {
        self * other.generic_vector()
    }
}
impl Mul<SparseVector> for Scalar {
    type Output = SparseVector;
    fn mul(self, rhs: SparseVector) -> Self::Output {
        self.mul_ga(rhs)
    }
}

impl DivGA<Scalar> for SparseVector {
    type Output = SparseVector;
    fn div_ga(self, other: Scalar) -> Self::Output {
        self.generic_vector() / other
    }
}
impl std::ops::Div<Scalar> for SparseVector {
    type Output = SparseVector;
    fn div(self, rhs: Scalar) -> Self::Output {
        self.div_ga(rhs)
    }
}

impl DivGA<SparseVector> for Scalar {
    type Output = SparseVector;
    fn div_ga(self, other: SparseVector) -> Self::Output {
        self / other.generic_vector()
    }
}
impl std::ops::Div<SparseVector> for Scalar {
    type Output = SparseVector;
    fn div(self, rhs: SparseVector) -> Self::Output {
        self.div_ga(rhs)
    }
}

impl MagGA for SparseVector {
    fn mag2_ga(&self) -> Scalar {
        self.generic_vector_ref().mag2()
    }
}
impl SparseVector {
    pub fn mag2(&self) -> Scalar {
        self.mag2_ga()
    }

    pub fn mag(&self) -> Scalar {
        self.mag_ga()
    }
}

impl DotRefGA<SparseVector> for SparseVector {
    type Output = Scalar;
    fn dot_ref_ga(self, other: &SparseVector) -> Self::Output {
        self.generic_vector()
            .dot_ref_ga(&other.generic_vector_ref())
    }
}
impl SparseVector {
    pub fn dot(self, other: &SparseVector) -> Scalar {
        self.dot_ref_ga(other)
    }
}
