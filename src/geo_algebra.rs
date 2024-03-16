pub trait GeoAlgebra<const P: usize, const N: usize>: Sized {
    type Data;
    type Index;
    type Translator;

    fn get(&self, index: Self::Index) -> Option<&Self::Data>;
    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Data>;

    fn add(&mut self, other: impl AsRef<Self>);
    fn sub(&mut self, other: impl AsRef<Self>);

    fn mul(&mut self, other: impl AsRef<Self>);

    fn try_div(&mut self, other: impl AsRef<Self>) -> Option<()>;

    fn try_inv(&mut self) -> Option<()>;

    fn translate(&mut self, translator: Self::Translator);
}

pub trait GeoAlgebraOtherOps<const P: usize, const N: usize> : GeoAlgebra<P, N> {
    fn div(&mut self, rhs: impl AsRef<Self>);

    fn inv(&mut self);
}

pub trait GeoCast<const P: usize, const N: usize>: GeoAlgebra<P, N> {
    fn cast<const P2: usize, const N2: usize, U: GeoAlgebra<P2, N2, Data = Self::Data>>(self) -> U;
}
