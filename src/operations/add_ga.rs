pub trait AddGA<T> {
    type Output;
    fn add_ga(self, other: &T) -> Self::Output;
}
