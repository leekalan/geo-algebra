use crate::{
    algebras::scalar::Scalar, iterate_values_ga::IterateValuesGA, operations::{add_ga::AddGA, div_ga::DivGA, mul_ga::MulGA, sub_ga::SubGA}, size_ga::SizeGA
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
