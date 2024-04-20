pub trait IndexSA<I> {
    fn at(&self, index: I) -> &f64;
}
pub trait IndexSAMut<I> {
    fn at_mut(&mut self, index: I) -> &mut f64;
}

pub trait TryIndexSA<I> {
    fn try_at(&self, index: I) -> Option<&f64>;
}
pub trait TryIndexSAMut<I> {
    fn try_at_mut(&mut self, index: I) -> Option<&mut f64>;
}
