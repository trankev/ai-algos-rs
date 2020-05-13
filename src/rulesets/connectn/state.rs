use super::plies;
use super::variants;
use crate::rulesets;
use crate::utils::bitarray;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State<Variant: variants::BaseVariant> {
    pub grids: [bitarray::BitArray<Variant::ArraySettings>; 2],
    pub current_player: u8,
}

impl<Variant: variants::BaseVariant> State<Variant> {
    pub fn new() -> State<Variant> {
        State {
            grids: [
                bitarray::BitArray::<Variant::ArraySettings>::zero(),
                bitarray::BitArray::<Variant::ArraySettings>::zero(),
            ],
            current_player: 0,
        }
    }

    pub fn from_indices(
        player1_indices: &[usize],
        player2_indices: &[usize],
        current_player: u8,
    ) -> State<Variant> {
        State {
            grids: [
                bitarray::BitArray::<Variant::ArraySettings>::from_indices(player1_indices),
                bitarray::BitArray::<Variant::ArraySettings>::from_indices(player2_indices),
            ],
            current_player,
        }
    }

    pub fn is_empty(&self, index: usize) -> bool {
        self.grids.iter().all(|grid| !grid.isset(index))
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

    pub fn swap(&self, grid_permutation: &[usize], switch_players: bool) -> Self {
        let permuted_grid1 = self.grids[0].swap(grid_permutation);
        let permuted_grid2 = self.grids[1].swap(grid_permutation);
        if switch_players {
            State {
                grids: [permuted_grid2, permuted_grid1],
                current_player: 1 - self.current_player,
            }
        } else {
            State {
                grids: [permuted_grid1, permuted_grid2],
                current_player: self.current_player,
            }
        }
    }
}

impl<Variant: variants::BaseVariant> rulesets::StateTrait for State<Variant> {
    fn current_player(&self) -> rulesets::Player {
        self.current_player
    }

    fn ascii_representation(&self) -> String {
        let mut result = String::new();
        for index in 0..Variant::GRID_SIZE {
            if self.grids[0].isset(index) {
                result.push('X');
            } else if self.grids[1].isset(index) {
                result.push('O');
            } else {
                result.push('.');
            }
            if index % Variant::GRID_SIZE == Variant::GRID_SIZE {
                result.push('\n');
            }
        }
        format!("{}\nTo play: {}", result, self.current_player())
    }
}

pub type TicTacToeState = State<variants::TicTacToe>;
pub type GomokuState = State<variants::Gomoku>;

#[cfg(test)]
mod tests {
    use super::super::plies;
    use super::super::variants;
    use super::super::variants::BaseVariant;
    use super::*;

    #[test]
    fn test_is_empty_empty() {
        let state = TicTacToeState::new();
        for index in 0..variants::TicTacToe::CELL_COUNT {
            assert!(state.is_empty(index));
        }
    }

    #[test]
    fn test_is_empty_filled() {
        let state = TicTacToeState::from_indices(&[4, 1], &[0, 8], 0);
        assert!(!state.is_empty(0));
        assert!(!state.is_empty(1));
        assert!(state.is_empty(2));
        assert!(!state.is_empty(8));
    }

    #[test]
    fn test_from_indices() {
        let from_indices = TicTacToeState::from_indices(&[4, 1], &[8, 7], 0);
        let mut from_scratch = TicTacToeState::new();
        for index in &[4, 8, 1, 7] {
            from_scratch.play(&plies::Ply { index: *index }).unwrap();
        }
        assert_eq!(from_indices, from_scratch);
    }
}
