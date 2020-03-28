use crate::utils::bitarray;
use crate::utils::bitarray::BitArray;

pub struct State {
    grids: [bitarray::BitArray9; 2],
    current_player: u8,
}

impl State {
    pub fn new() -> State {
        State {
            grids: [bitarray::BitArray9::zero(); 2],
            current_player: 0,
        }
    }
}
