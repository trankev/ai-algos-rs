use super::plies;
use crate::rulesets;
use crate::utils::bitarray;
use crate::utils::bitarray::BitArray;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn from_indices(
        player1_indices: &[usize],
        player2_indices: &[usize],
        current_player: u8,
    ) -> State {
        State {
            grids: [
                bitarray::BitArray9::from_indices(player1_indices),
                bitarray::BitArray9::from_indices(player2_indices),
            ],
            current_player,
        }
    }

    pub fn is_empty(&self, index: usize) -> bool {
        self.grids.iter().all(|&grid| !grid.isset(index))
    }

    pub fn play(&mut self, ply: &plies::Ply) -> Result<(), rulesets::PlayError> {
        for grid in &self.grids {
            if grid.isset(ply.index as usize) {
                return Err(rulesets::PlayError {
                    message: "Cell is occupied",
                    field: "index",
                });
            }
        }
        self.grids[self.current_player as usize].set(ply.index as usize);
        self.current_player = 1 - self.current_player;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::plies;
    use super::State;

    #[test]
    fn test_is_empty_empty() {
        let state = State::new();
        for index in 0..9 {
            assert!(state.is_empty(index));
        }
    }

    #[test]
    fn test_is_empty_filled() {
        let state = State::from_indices(&[4, 1], &[0, 8], 0);
        assert!(!state.is_empty(0));
        assert!(!state.is_empty(1));
        assert!(state.is_empty(2));
        assert!(!state.is_empty(8));
    }

    #[test]
    fn test_from_indices() {
        let from_indices = State::from_indices(&[4, 1], &[8, 7], 0);
        let mut from_scratch = State::new();
        for index in &[4, 8, 1, 7] {
            from_scratch.play(&plies::Ply { index: *index }).unwrap();
        }
        assert_eq!(from_indices, from_scratch);
    }
}
