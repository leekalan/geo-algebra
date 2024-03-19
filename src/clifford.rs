use crate::geo_algebra::GA;

#[derive(Debug, Clone, Copy)]
pub struct CliffordType {
    pub pos: usize,
    pub neg: usize,
    pub nul: usize,
}
impl CliffordType {
    pub fn new(pos: usize, neg: usize, nul: usize) -> Self {
        CliffordType { pos, neg, nul }
    }
    pub fn consume(&self) -> Option<Self> {
        if self.pos > 0 {
            Some(CliffordType {
                pos: self.pos - 1,
                neg: self.neg,
                nul: self.nul,
            })
        } else if self.neg > 0 {
            Some(CliffordType {
                pos: self.pos,
                neg: self.neg - 1,
                nul: self.nul,
            })
        } else if self.nul > 0 {
            Some(CliffordType {
                pos: self.pos,
                neg: self.neg,
                nul: self.nul - 1,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct CliffordObject {
    clifford_type: CliffordType,
    scalar: f32,
    data: Box<[CliffordObject]>,
}
impl CliffordObject {
    pub fn new<T: Into<CliffordType>>(clifford_type: T) -> Self {
        let clifford_type = clifford_type.into();
        let count = clifford_type.pos + clifford_type.neg + clifford_type.nul;

        let mut data = Vec::with_capacity(count);

        let mut new_type = clifford_type;
        for _ in 0..count {
            new_type = new_type.consume().unwrap();
            let child = CliffordObject::new(new_type);
            data.push(child);
        }
        
        CliffordObject {
            clifford_type,
            scalar: 0f32,
            data: data.into_boxed_slice(),
        }
    }
}
impl GA for CliffordObject {
    fn get(&self, index: &[usize]) -> Option<f32> {
        let k = match index.first() {
            Some(v) => *v,
            None => return Some(self.scalar),
        };
        self.data.get(k)?.get(&index[1..])
    }
}
