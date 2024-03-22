use crate::compose::{Compose, ComposeRef, ComposeSelf, ComposeSelfRef};

pub trait Inject<T, R> {
    fn inject(self, container: T) -> R;
}

pub trait InjectInto<T> {
    fn inject_into(self, container: &mut T);
}
pub trait InjectRef<'a, T, R>: Sized {
    fn inject_ref(&'a self, container: T) -> R;
}

pub trait InjectIntoRef<'a, T> {
    fn inject_into_ref(&'a self, container: &mut T);
}

impl<T: Compose<U, R>, U, R> Inject<T, R> for U {
    fn inject(self, container: T) -> R {
        container.compose(self)
    }
}

impl<T: ComposeSelf<U>, U> InjectInto<T> for U {
    fn inject_into(self, container: &mut T) {
        container.compose_self(self);
    }
}

impl<'a, T: ComposeRef<'a, U, R>, U, R> InjectRef<'a, T, R> for U {
    fn inject_ref(&'a self, container: T) -> R {
        container.compose_ref(self)
    }
}

impl<'a, T: ComposeSelfRef<'a, U>, U> InjectIntoRef<'a, T> for U {
    fn inject_into_ref(&self, container: &mut T) {
        container.compose_self_ref(self);
    }
}
