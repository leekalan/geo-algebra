use super::GA;

/// The "multiplicative inverse" of the geometric algebra
pub trait Inversible: GA {
    /// The "multiplicative inverse" of the geometric algebra
    fn inverse(&self) -> Option<Self>;
}
