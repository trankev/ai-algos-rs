use std::ops;

pub trait BitArray: ops::BitAnd<Self> + ops::BitOr<Self> + ops::BitXor<Self> + Clone + Copy + PartialEq {
    type Index;
    fn zero() -> Self;
    fn from_indices(indices: &[Self::Index]) -> Self;
    fn isset(&self, index: Self::Index) -> bool;
    fn set(&mut self, index: Self::Index);
}
