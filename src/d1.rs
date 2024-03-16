use crate::{geo_algebra::{GeoAlgebra, GeoAlgebraOtherOps}, scalar::Scalar};

pub enum Index {
    Scalar,
    A,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct D1 {
    scalar: f64,
    a: f64,
}

impl GeoAlgebra<1, 0> for D1 {
    type Data = f64;
    type Index = Index;
    type Translator = ();
    
    fn get(&self, index: Self::Index) -> Option<&Self::Data> {
        match index {
            Index::Scalar => Some(&self.scalar),
            Index::A => Some(&self.a),
        }
    }
    
    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Data> {
        match index {
            Index::Scalar => Some(&mut self.scalar),
            Index::A => Some(&mut self.a),
        }
    }
    
    fn add(&mut self, other: impl AsRef<Self>) {
        let other = other.as_ref();

        self.scalar += other.scalar;
        self.a += other.a;
    }
    
    fn sub(&mut self, other: impl AsRef<Self>) {
        let other = other.as_ref();

        self.scalar -= other.scalar;
        self.a -= other.a;
    }
    
    fn mul(&mut self, other: impl AsRef<Self>) {
        let other = other.as_ref();
        
        let s1 = self.scalar * other.a + self.a * other.scalar;

        self.scalar = (self.scalar + self.a) * (other.scalar + other.a) - s1;
        self.a = s1;
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
impl GeoAlgebraOtherOps<1, 0> for D1 {
    fn div(&mut self, other: impl AsRef<Self>) {
        let other = other.as_ref();
        
        let denominator = self.a.powi(2) - other.a.powi(2);

        let mut numerator = D1 { scalar: other.scalar, a: -other.a };
        numerator.mul(&self);
        
        self.scalar = numerator.scalar / denominator;
        self.a = numerator.a / denominator;
    }
    
    fn inv(&mut self) {
        todo!()
    }
}

impl From<f64> for D1 {
    fn from(scalar: f64) -> Self {
        D1 { scalar, a: 0f64 }
    }
}
impl From<Scalar> for D1 {
    fn from(scalar: Scalar) -> Self {
        D1 { scalar: *scalar.get(()).unwrap(), a: 0f64}
    }
}
impl AsRef<D1> for D1 {
    fn as_ref(&self) -> &D1 {
        self
    }
}

crate::op_macro::generate_ops_impl!(D1);
crate::op_macro::generate_other_ops_impl!(D1);