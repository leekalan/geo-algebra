use crate::algebras::scalar::Scalar;

pub trait MagGA {
    fn mag2_ga(&self) -> Scalar;
    fn mag_ga(&self) -> Scalar {
        Scalar::new(self.mag2_ga().internal().sqrt())
    }
}
