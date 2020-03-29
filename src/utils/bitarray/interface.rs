use std::ops;

pub trait BitArray: Clone + Copy + ops::BitOr<Self> + PartialEq {
    type Index;
    fn zero() -> Self;
    fn from_indices(indices: &[Self::Index]) -> Self;
    fn isset(&self, index: Self::Index) -> bool;
    fn set(&mut self, index: Self::Index);
}
