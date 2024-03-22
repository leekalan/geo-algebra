pub mod clifford;
pub mod compose;
pub mod inject;
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

    #[test]
    fn test1() {
        let clifford = CliffordObject::new(CliffordType::new(1, 0, 0));
        println!("{:?}", clifford.get(&[0]))
    }

    #[test]
    fn test2() {
        let clifford = CliffordObject::new(CliffordType::new(2, 0, 0));
        let m_c = clifford.shallow_map(ShallowMap::new(HashMap::from([(0, 1), (1, 0)])));
        let m2_c = m_c.shallow_map(ShallowMap::new(HashMap::from([(1, 2), (2, 0), (0, 1)])));
        let m_c_collapsed = m2_c.clone();
        println!("{:?}\n{:?}\n{:?}", m_c, m2_c, m_c_collapsed);
    }
}
