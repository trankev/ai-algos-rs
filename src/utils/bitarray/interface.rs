use std::ops;

pub trait BitArray<'a, 'b, 'c, 'd>: Clone + Copy + PartialEq
where Self: 'a + 'b + 'c + 'd, &'a Self: ops::BitAnd<&'b Self> + ops::BitOr<&'c Self> + ops::BitXor<&'d Self> {
    type Index;
    fn zero() -> Self;
    fn from_indices(indices: &[Self::Index]) -> Self;
    fn isset(&self, index: Self::Index) -> bool;
    fn set(&mut self, index: Self::Index);
}
