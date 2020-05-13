use crate::utils::bitarray;
use std::fmt;
use std::hash;

pub trait BaseVariant:
    Clone + hash::Hash + fmt::Debug + Eq + Ord + PartialEq + PartialOrd + Send
{
    type ArraySettings: bitarray::BitArraySettings;

    const GRID_SIZE: usize;
    const CELL_COUNT: usize = Self::GRID_SIZE * Self::GRID_SIZE;
    const PLAYER_POSITIONS: [&'static [usize]; 2];
}
