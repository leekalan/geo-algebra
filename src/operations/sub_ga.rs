pub trait SubGA<T> {
    type Output;
    fn sub_ga(self, other: &T) -> Self::Output;
}
