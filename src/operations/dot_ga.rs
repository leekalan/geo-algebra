pub trait DotGA<T> {
    type Output;
    fn dot_ga(self, other: T) -> Self::Output;
}

pub trait DotRefGA<T> {
    type Output;
    fn dot_ref_ga(self, other: &T) -> Self::Output;
}
