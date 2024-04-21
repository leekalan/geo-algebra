pub trait DivGA<T> {
    type Output;
    fn div_ga(self, other: T) -> Self::Output;
}

pub trait DivRefGA<T> {
    type Output;
    fn div_ref_ga(self, other: &T) -> Self::Output;
}
