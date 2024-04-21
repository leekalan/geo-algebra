pub trait SubGA<T> {
    type Output;
    fn sub_ga(self, other: T) -> Self::Output;
}

pub trait SubRefGA<T> {
    type Output;
    fn sub_ref_ga(self, other: &T) -> Self::Output;
}
