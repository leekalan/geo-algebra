pub trait MulGA<T> {
    type Output;
    fn mul_ga(self, other: T) -> Self::Output;
}

pub trait MulRefGA<T> {
    type Output;
    fn mul_ref_ga(self, other: &T) -> Self::Output;
}
