use crate::algebras::scalar::Scalar;

pub trait Abs: Sized {
    fn abs2(self) -> Scalar;
    fn abs(self) -> Scalar {
        Scalar::new(self.abs2().sqrt())
    }
}

pub trait Dot<T> {
    type Output;

    fn dot(self, other: T) -> Self::Output;
}

pub trait Inv {
    type Output;

    fn inv(self) -> Self::Output;
}

pub trait InvAssign {
    fn inv_assign(&mut self);
}
