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

    pub fn isempty(&self, index: u8) -> bool {
        self.grids.iter().all(|&grid| !grid.isset(index))
    }
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn test_is_empty_empty() {
        let state = State::new();
        for index in 0..9 {
            assert!(state.isempty(index));
        }
    }
}
