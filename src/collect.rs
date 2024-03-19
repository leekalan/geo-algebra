use crate::{deep_map::DeepMappedGA, geo_algebra::GA, shallow_map::ShallowMappedGA};

pub trait CollectFromShallow<T>: GA {
    fn collect_from_shallow(mapped: &ShallowMappedGA<Self>) -> T;
}

pub trait CollectFromDeep<T>: GA {
    fn collect_from_deep(mapped: &DeepMappedGA<Self>) -> T;
}
