pub mod clifford;
pub mod deep_map;
pub mod geo_algebra;
pub mod shallow_map;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use self::shallow_map::ShallowMap;

    use super::*;

    use clifford::{CliffordObject, CliffordType};
    use geo_algebra::GA;
    use wrappr::composer::Composer;

    #[test]
    fn test1() {
        let clifford = CliffordObject::new(CliffordType::new(1, 0, 0));
        println!("{:?}", clifford.get(&[0]))
    }

    #[test]
    fn shallow_map_test() {
        let map1 = ShallowMap::new(HashMap::from([(0, 1), (1, 0)]));
        let map2 = ShallowMap::new(HashMap::from([(0, 1), (1, 2), (2, 0)]));

        let mut composed_map = map2.clone();
        composed_map.compose(map1.clone());

        println!("{:?}\n{:?}\n{:?}", map1, map2, composed_map);
    }
}
