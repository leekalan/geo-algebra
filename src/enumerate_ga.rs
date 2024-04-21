pub trait EnumerateGA<I> {
    fn enumerate(&self) -> impl Iterator<Item = (I, &f64)>;
    fn enumerate_mut(&mut self) -> impl Iterator<Item = (I, &mut f64)>;
    fn into_enumerate(self) -> impl Iterator<Item = (I, f64)>;
}

pub trait EnumerateAndSortGA<I>: EnumerateGA<I> {
    fn enumerate_and_sort(&self) -> impl Iterator<Item = (I, &f64)>;
    fn enumerate_and_sort_mut(&mut self) -> impl Iterator<Item = (I, &mut f64)>;
    fn into_enumerate_and_sort(self) -> impl Iterator<Item = (I, f64)>;
}
