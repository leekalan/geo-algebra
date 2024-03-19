use crate::{geo_algebra::GA, geo_map::MappedGA};

pub trait CollectFrom: GA {
    fn collect_from(mapped: MappedGA<Self>) -> Self;
}
