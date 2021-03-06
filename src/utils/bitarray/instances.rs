use super::settings;

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BitArray225Settings {}
impl settings::BitArraySettings for BitArray225Settings {
    const SIZE: usize = 225;
    type FirstBitType = u64;
    type ArrayLength = typenum::U3;
    type LastBitType = u64;
}

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BitArray9Settings {}
impl settings::BitArraySettings for BitArray9Settings {
    const SIZE: usize = 9;
    type FirstBitType = u64;
    type ArrayLength = typenum::U0;
    type LastBitType = u16;
}

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BitArray64Settings {}
impl settings::BitArraySettings for BitArray64Settings {
    const SIZE: usize = 64;
    type FirstBitType = u64;
    type ArrayLength = typenum::U0;
    type LastBitType = u64;
}

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BitArray36Settings {}
impl settings::BitArraySettings for BitArray36Settings {
    const SIZE: usize = 36;
    type FirstBitType = u64;
    type ArrayLength = typenum::U0;
    type LastBitType = u64;
}

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BitArray16Settings {}
impl settings::BitArraySettings for BitArray16Settings {
    const SIZE: usize = 16;
    type FirstBitType = u64;
    type ArrayLength = typenum::U0;
    type LastBitType = u16;
}
