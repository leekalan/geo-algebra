pub trait IterateValuesGA {
    fn iterate_values(&self) -> impl Iterator<Item = &f64>;
    fn iterate_values_mut(&mut self) -> impl Iterator<Item = &mut f64>;
    fn into_iterate_values(self) -> impl Iterator<Item = f64>;
}