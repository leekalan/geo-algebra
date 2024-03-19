pub mod clifford;
pub mod collect;
pub mod geo_algebra;
pub mod geo_map;

#[cfg(test)]
mod tests {
    use super::*;

    use clifford::{CliffordObject, CliffordType};
    use geo_algebra::GA;

    #[test]
    fn test() {
        let clifford = CliffordObject::new(CliffordType::new(1, 0, 0));
        println!("{:?}", clifford.get(&[0]))
    }
}
