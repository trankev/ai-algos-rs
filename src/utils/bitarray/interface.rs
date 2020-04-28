use std::ops;

pub trait BitArray<'a>: Clone + Copy + PartialEq
where
    Self: 'a,
    &'a Self: ops::BitAnd<&'a Self> + ops::BitOr<&'a Self> + ops::BitXor<&'a Self>,
{
    type Index;
    fn zero() -> Self;
    fn from_indices(indices: &[Self::Index]) -> Self;
    fn isset(&self, index: Self::Index) -> bool;
    fn set(&mut self, index: Self::Index);
}
