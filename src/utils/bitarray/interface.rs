use std::fmt;
use std::hash;
use std::ops;

pub enum MaskComparison {
    Equal,
    Partial,
    Zero,
}

pub trait BitArray:
    Clone + Copy + fmt::Debug + Eq + hash::Hash + Ord + PartialEq + PartialOrd + Send
where
    Self: ops::BitAnd<Self, Output = Self>
        + ops::BitOr<Self, Output = Self>
        + ops::BitXor<Self, Output = Self>,
    for<'a> Self: ops::BitAnd<&'a Self, Output = Self>
        + ops::BitOr<&'a Self, Output = Self>
        + ops::BitXor<&'a Self, Output = Self>,
    for<'a> &'a Self: ops::BitAnd<Self, Output = Self>
        + ops::BitOr<Self, Output = Self>
        + ops::BitXor<Self, Output = Self>,
    for<'a, 'b> &'a Self: ops::BitAnd<&'b Self, Output = Self>
        + ops::BitOr<&'b Self, Output = Self>
        + ops::BitXor<&'b Self, Output = Self>,
{
    fn zero() -> Self;
    fn isset(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
    fn compare_with_mask(&self, mask: &Self) -> MaskComparison;

    fn from_indices(indices: &[usize]) -> Self {
        let mut result = Self::zero();
        for index in indices {
            result.set(*index);
        }
        result
    }

    fn swap(&self, permutation: &[usize]) -> Self {
        let mut result = Self::zero();
        for (index, &target) in permutation.iter().enumerate() {
            if self.isset(index) {
                result.set(target);
            }
        }
        result
    }
}
