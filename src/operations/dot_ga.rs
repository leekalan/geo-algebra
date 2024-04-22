pub trait DotGA<T> {
    type Output;
    fn dot_ga(&self, other: &T) -> Self::Output;
}
