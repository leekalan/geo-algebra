pub trait Compose<U, R> {
    fn compose(self, contents: U) -> R;
}

pub trait ComposeSelf<U> {
    fn compose_self(&mut self, contents: U);
}

pub trait ComposeRef<U, R> {
    fn compose_ref(self, contents: &U) -> R;
}

pub trait ComposeSelfRef<U> {
    fn compose_self_ref(&mut self, contents: &U);
}

impl<T: ComposeSelf<U>, U> Compose<U, T> for T {
    fn compose(self, contents: U) -> T {
        let mut this = self;
        this.compose_self(contents);
        this
    }
}

impl<T: ComposeSelfRef<U>, U> ComposeRef<U, T> for T {
    fn compose_ref(self, contents: &U) -> T {
        let mut this = self;
        this.compose_self_ref(contents);
        this
    }
}