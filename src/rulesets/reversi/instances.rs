use super::variants;
use crate::utils::bitarray;

#[derive(Clone, Copy, Hash, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Classic {}
impl variants::BaseVariant for Classic {
    type ArraySettings = bitarray::BitArray64Settings;
    const GRID_SIZE: usize = 8;
    const PLAYER_POSITIONS: [&'static [usize]; 2] = [&[27, 36], &[28, 35]];
}

#[derive(Clone, Copy, Hash, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Mini {}
impl variants::BaseVariant for Mini {
    type ArraySettings = bitarray::BitArray36Settings;
    const GRID_SIZE: usize = 6;
    const PLAYER_POSITIONS: [&'static [usize]; 2] = [&[14, 21], &[15, 20]];
}

#[derive(Clone, Copy, Hash, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Micro {}
impl variants::BaseVariant for Micro {
    type ArraySettings = bitarray::BitArray16Settings;
    const GRID_SIZE: usize = 4;
    const PLAYER_POSITIONS: [&'static [usize]; 2] = [&[5, 10], &[6, 9]];
}
