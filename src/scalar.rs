use std::borrow::Borrow;

use crate::geo_algebra::{GeoAlgebra, GeoAlgebraOtherOps};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Scalar {
    value: f64,
}

impl GeoAlgebra<0, 0> for Scalar {
    type Data = f64;
    type Index = ();
    type Translator = ();

    fn get(&self, _index: Self::Index) -> Option<&Self::Data> {
        Some(&self.value)
    }

    fn get_mut(&mut self, _index: Self::Index) -> Option<&mut Self::Data> {
        Some(&mut self.value)
    }

    fn add(&mut self, other: impl AsRef<Self>) {
        self.value += other.as_ref().value;
    }

    fn sub(&mut self, other: impl AsRef<Self>) {
        self.value -= other.as_ref().value;
    }

    fn mul(&mut self, other: impl AsRef<Self>) {
        self.value *= other.as_ref().value;
    }

    fn try_div(&mut self, other: impl AsRef<Self>) -> Option<()> {
        self.div(other);
        Some(())
    }

    fn try_inv(&mut self) -> Option<()> {
        self.inv();
        Some(())
    }

    fn translate(&mut self, _translator: Self::Translator) {}
}
impl GeoAlgebraOtherOps<0, 0> for Scalar {
    fn div(&mut self, other: impl AsRef<Self>) {
        self.value /= other.as_ref().value;
    }

    fn inv(&mut self) {
        self.value = 1f64 / self.value;
    }
}

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Scalar { value }
    }
}
impl Borrow<f64> for Scalar {
    fn borrow(&self) -> &f64 {
        &self.value
    }
}
impl AsRef<f64> for Scalar {
    fn as_ref(&self) -> &f64 {
        &self.value
    }
}
impl AsRef<Scalar> for Scalar {
    fn as_ref(&self) -> &Scalar {
        self
    }
}

crate::op_macro::generate_ops_impl!(Scalar);
crate::op_macro::generate_other_ops_impl!(Scalar);