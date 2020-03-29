use crate::utils::bitarray;
use crate::utils::bitarray::BitArray;
use super::plies;
use crate::rulesets;

#[derive(Clone, Debug)]
pub struct State {
    pub grids: [bitarray::BitArray9; 2],
    pub current_player: u8,
}

impl State {
    pub fn new() -> State {
        State {
            grids: [bitarray::BitArray9::zero(); 2],
            current_player: 0,
        }
    }

    pub fn from_indices(player1_indices: &[u8], player2_indices: &[u8], current_player: u8) -> State {
        State {
            grids: [
                bitarray::BitArray9::from_indices(player1_indices),
                bitarray::BitArray9::from_indices(player2_indices),
            ],
            current_player
        }
    }

    pub fn isempty(&self, index: u8) -> bool {
        self.grids.iter().all(|&grid| !grid.isset(index))
    }

    pub fn play(&mut self, ply: &plies::Ply) -> Result<(), rulesets::PlayError> {
        for grid in &self.grids {
            if grid.isset(ply.index) {
                return Err(rulesets::PlayError{
                    message: "Cell is occupied",
                    field: "index",
                });
            }
        }
        self.grids[self.current_player as usize].set(ply.index);
        self.current_player = 1 - self.current_player;
        Ok(())
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
