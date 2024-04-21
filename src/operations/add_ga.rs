pub trait AddGA<T> {
    type Output;
    fn add_ga(self, other: T) -> Self::Output;
}

pub trait AddRefGA<T> {
    type Output;
    fn add_ref_ga(self, other: &T) -> Self::Output;
}
