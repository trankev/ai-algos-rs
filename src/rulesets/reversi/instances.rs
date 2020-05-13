use super::variants;
use crate::utils::bitarray;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Classic {}

impl variants::BaseVariant for Classic {
    type ArraySettings = bitarray::BitArray64Settings;
    const GRID_SIZE: usize = 8;
    const PLAYER_POSITIONS: [&'static [usize]; 2] = [&[28, 37], &[29, 36]];
}
