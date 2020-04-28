use std::ops;

pub trait BitArray: Clone + Copy + PartialEq
where
    Self: ops::BitAnd<Self> + ops::BitOr<Self> + ops::BitXor<Self>,
    for<'a> Self: ops::BitAnd<&'a Self> + ops::BitOr<&'a Self> + ops::BitXor<&'a Self>,
    for<'a> &'a Self: ops::BitAnd<Self> + ops::BitOr<Self> + ops::BitXor<Self>,
    for<'a, 'b> &'a Self: ops::BitAnd<&'b Self> + ops::BitOr<&'b Self> + ops::BitXor<&'b Self>,
{
    type Index;
    fn zero() -> Self;
    fn from_indices(indices: &[Self::Index]) -> Self;
    fn isset(&self, index: Self::Index) -> bool;
    fn set(&mut self, index: Self::Index);
}
