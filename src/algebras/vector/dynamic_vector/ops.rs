use crate::{
    algebras::scalar::Scalar,
    iterate_values_ga::IterateValuesGA,
    operations::{
        add_ga::AddGA, div_ga::DivGA, dot_ga::DotGA, inv_ga::InvGA, mag_ga::MagGA, mul_ga::MulGA,
        neg_ga::NegGA, sub_ga::SubGA,
    },
    size_ga::SizeGA,
};

use super::DynamicVector;

impl AddGA<DynamicVector> for DynamicVector {
    type Output = DynamicVector;
    fn add_ga(self, other: DynamicVector) -> Self::Output {
        let (mut lhs, rhs) = if self.size() > other.size() {
            (self, other)
        } else {
            (other, self)
        };

        for (lhs, rhs) in lhs.iterate_values_mut().zip(rhs.into_iterate_values()) {
            *lhs += rhs
        }

        lhs
    }
}
impl std::ops::Add<DynamicVector> for DynamicVector {
    type Output = DynamicVector;
    fn add(self, other: DynamicVector) -> Self::Output {
        self.add_ga(other)
    }
}

impl SubGA<DynamicVector> for DynamicVector {
    type Output = DynamicVector;
    fn sub_ga(self, other: DynamicVector) -> Self::Output {
        let (mut lhs, rhs) = if self.size() > other.size() {
            (self, other)
        } else {
            (other, self)
        };

        for (lhs, rhs) in lhs.iterate_values_mut().zip(rhs.into_iterate_values()) {
            *lhs -= rhs
        }

        lhs
    }
}
impl std::ops::Sub<DynamicVector> for DynamicVector {
    type Output = DynamicVector;
    fn sub(self, other: DynamicVector) -> Self::Output {
        self.sub_ga(other)
    }
}

impl NegGA for DynamicVector {
    fn neg_ga(&mut self) {
        for value in self.iterate_values_mut() {
            *value = -(*value);
        }
    }
}
impl std::ops::Neg for DynamicVector {
    type Output = DynamicVector;
    fn neg(mut self) -> Self::Output {
        self.neg_ga();
        self
    }
}

impl MulGA<Scalar> for DynamicVector {
    type Output = DynamicVector;
    fn mul_ga(mut self, other: Scalar) -> Self::Output {
        for value in self.iterate_values_mut() {
            *value *= other.internal();
        }
        self
    }
}
impl std::ops::Mul<Scalar> for DynamicVector {
    type Output = DynamicVector;
    fn mul(self, other: Scalar) -> Self::Output {
        self.mul_ga(other)
    }
}

impl MulGA<DynamicVector> for Scalar {
    type Output = DynamicVector;
    fn mul_ga(self, mut other: DynamicVector) -> Self::Output {
        for value in other.iterate_values_mut() {
            *value *= self.internal();
        }
        other
    }
}
impl std::ops::Mul<DynamicVector> for Scalar {
    type Output = DynamicVector;
    fn mul(self, other: DynamicVector) -> Self::Output {
        self.mul_ga(other)
    }
}

impl InvGA for DynamicVector {
    type Output = DynamicVector;
    fn inv_ga(self) -> Self::Output {
        let denominator = self.mag2();
        self / denominator
    }
}

impl DivGA<Scalar> for DynamicVector {
    type Output = DynamicVector;
    fn div_ga(mut self, other: Scalar) -> Self::Output {
        for value in self.iterate_values_mut() {
            *value /= other.internal();
        }
        self
    }
}
impl std::ops::Div<Scalar> for DynamicVector {
    type Output = DynamicVector;
    fn div(self, other: Scalar) -> Self::Output {
        self.div_ga(other)
    }
}

impl DivGA<DynamicVector> for Scalar {
    type Output = DynamicVector;
    fn div_ga(self, other: DynamicVector) -> Self::Output {
        let mult = self / other.mag2();
        other * mult
    }
}
impl std::ops::Div<DynamicVector> for Scalar {
    type Output = DynamicVector;
    fn div(self, other: DynamicVector) -> Self::Output {
        self.div_ga(other)
    }
}

impl MagGA for DynamicVector {
    fn mag2_ga(&self) -> Scalar {
        let mut scalar = 0.;
        for value in self.iterate_values() {
            scalar += value * value;
        }
        Scalar::new(scalar)
    }
}
impl DynamicVector {
    pub fn mag2(&self) -> Scalar {
        self.mag2_ga()
    }

    pub fn mag(&self) -> Scalar {
        self.mag_ga()
    }
}

impl DotGA<DynamicVector> for DynamicVector {
    type Output = Scalar;
    fn dot_ga(self, other: DynamicVector) -> Self::Output {
        let mut scalar = 0.;
        for (lhs, rhs) in self.into_iterate_values().zip(other.into_iterate_values()) {
            scalar += lhs * rhs;
        }
        Scalar::new(scalar)
    }
}
impl DynamicVector {
    pub fn dot(self, other: DynamicVector) -> Scalar {
        self.dot_ga(other)
    }
}
