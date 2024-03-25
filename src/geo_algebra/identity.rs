use super::GA;

/// The "multiplicative identity" of the geometric algebra
pub trait Identity: GA {
    /// The "multiplicative identity" of the geometric algebra
    fn identity(&self) -> Self;
}
