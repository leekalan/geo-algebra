use std::{
    borrow::Cow,
    ops::{Div, Mul},
};

use crate::{
    algebras::scalar::Scalar,
    operations::{Abs, Dot},
};

use super::Vectorize;

pub fn proj<T, U: Clone>(from: T, onto: Cow<'_, U>) {
    todo!()
}
