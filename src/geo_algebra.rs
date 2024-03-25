pub mod dot_product;
pub mod identity;
pub mod inversible;
pub mod multiply;
pub mod sum;
pub mod create_shallow_map;
pub mod create_deep_map;

use std::borrow::Borrow;

use wrappr::injectable::InjectableRef;

pub trait GA: Sized {
    fn get_multi(&self, index: &[usize]) -> Option<f32>;
}
