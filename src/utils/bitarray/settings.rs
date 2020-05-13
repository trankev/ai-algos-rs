use std::fmt;
use std::hash;
use std::mem;

pub trait BitArraySettings: fmt::Debug + Ord + hash::Hash + Clone {
    const SIZE: usize;
    type FirstBitType: num::PrimInt + Default + fmt::Debug + hash::Hash + Send;
    const INTEGER_SIZE: usize = 8 * mem::size_of::<Self::FirstBitType>();
    type ArrayLength: generic_array::ArrayLength<Self::FirstBitType>
        + Ord
        + fmt::Debug
        + hash::Hash
        + Send;
    type LastBitType: num::PrimInt + Default + Send + fmt::Debug + hash::Hash + Send;
}
