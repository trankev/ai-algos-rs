use crate::utils::bitarray;
use std::fmt;
use std::hash;

pub trait BaseVariant: Clone + Copy + Send + Ord + fmt::Debug + hash::Hash {
    type ArraySettings: bitarray::BitArraySettings;

    const GRID_SIZE: usize;
    const CELL_COUNT: usize = Self::GRID_SIZE * Self::GRID_SIZE;
    const RUN_COUNT: usize;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Gomoku {}

impl BaseVariant for Gomoku {
    type ArraySettings = bitarray::BitArray225Settings;

    const GRID_SIZE: usize = 15;
    const RUN_COUNT: usize = 5;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TicTacToe {}

impl BaseVariant for TicTacToe {
    type ArraySettings = bitarray::BitArray9Settings;

    const GRID_SIZE: usize = 3;
    const RUN_COUNT: usize = 3;
}
