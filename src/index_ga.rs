pub trait IndexGA<I> {
    fn at(&self, index: I) -> &f64;
}
pub trait IndexGAMut<I> {
    fn at_mut(&mut self, index: I) -> &mut f64;
}

pub trait TryIndexGA<I> {
    fn try_at(&self, index: I) -> Option<&f64>;
}
pub trait TryIndexGAMut<I> {
    fn try_at_mut(&mut self, index: I) -> Option<&mut f64>;
}
