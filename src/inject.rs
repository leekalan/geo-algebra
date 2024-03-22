use crate::compose::{Compose, ComposeRef, ComposeSelf, ComposeSelfRef};

pub trait Inject<T, R> {
    fn inject(self, container: T) -> R;
}

pub trait InjectInto<T> {
    fn inject_into(self, container: &mut T);
}

pub trait InjectRef<T, R> : Sized {
    fn inject_ref(&self, container: T) -> R;
}

pub trait InjectIntoRef<T> {
    fn inject_into_ref(&self, container: &mut T);
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

impl<T: ComposeRef<U, R>, U, R> InjectRef<T, R> for U {
    fn inject_ref(&self, container: T) -> R {
        container.compose_ref(self)
    }
}

impl<T: ComposeSelfRef<U>, U> InjectIntoRef<T> for U {
    fn inject_into_ref(&self, container: &mut T) {
        container.compose_self_ref(self);
    }
}