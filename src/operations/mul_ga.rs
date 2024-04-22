pub trait MulGA<T> {
    type Output;
    fn mul_ga(self, other: T) -> Self::Output;
}
