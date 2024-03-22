pub mod clifford;
pub mod compose;
pub mod deep_map;
pub mod geo_algebra;
pub mod inject;
pub mod shallow_map;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{compose::Compose, inject::Inject};

    use self::shallow_map::ShallowMap;

    use super::*;

    use clifford::{CliffordObject, CliffordType};
    use geo_algebra::GA;

    #[test]
    fn test1() {
        let clifford = CliffordObject::new(CliffordType::new(1, 0, 0));
        println!("{:?}", clifford.get(&[0]))
    }

    #[test]
    fn shallow_map_test() {
        let map1 = ShallowMap::new(HashMap::from([(0, 1), (1, 0)]));
        let map2 = ShallowMap::new(HashMap::from([(0, 1), (1, 2), (2, 0)]));

        println!("{:?}\n{:?}", map1, map2);

        let composed_map = map2.compose(map1);

        println!("{:?}", composed_map);
    }
}
