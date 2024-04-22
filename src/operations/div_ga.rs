pub trait DivGA<T> {
    type Output;
    fn div_ga(self, other: T) -> Self::Output;
}
