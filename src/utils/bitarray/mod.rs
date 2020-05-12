mod array225;
mod array9;
mod generic;
mod interface;

pub use array225::BitArray225;
pub use array9::BitArray9;
pub use interface::BitArray;
pub use interface::MaskComparison;

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BitArray225Settings {}
impl generic::BitArraySettings for BitArray225Settings {
    const SIZE: usize = 225;
    type FirstBitType = u64;
    type ArrayLength = typenum::U3;
    type LastBitType = u64;
}

// pub type BitArray225 = generic::GenericBitArray<BitArray225Settings>;
