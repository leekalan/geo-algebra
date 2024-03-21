pub trait Inject<U> {
    fn inject(self, target: U) -> U;
}

pub trait Compose<T> {
    fn compose(&mut self, target: T);
}

impl<T, U: Compose<T>> Inject<U> for T {
    fn inject(self, target: U) -> U {
        let mut target = target;
        target.compose(self);
        target
    }
}