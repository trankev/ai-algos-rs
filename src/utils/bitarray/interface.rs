use std::ops;

pub trait BitArray: Clone + Copy + PartialEq
where
    Self: ops::BitAnd<Self> + ops::BitOr<Self> + ops::BitXor<Self>,
    for<'a> Self: ops::BitAnd<&'a Self> + ops::BitOr<&'a Self> + ops::BitXor<&'a Self>,
    for<'a> &'a Self: ops::BitAnd<Self> + ops::BitOr<Self> + ops::BitXor<Self>,
    for<'a, 'b> &'a Self: ops::BitAnd<&'b Self> + ops::BitOr<&'b Self> + ops::BitXor<&'b Self>,
{
    fn zero() -> Self;
    fn from_indices(indices: &[usize]) -> Self;
    fn isset(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
}
