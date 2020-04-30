use super::plies;
use crate::rulesets;
use crate::utils::bitarray;
use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct State<ArrayType>
where
    ArrayType: bitarray::BitArray,
    for<'a> ArrayType: ops::BitAnd<&'a ArrayType, Output = ArrayType>
        + ops::BitOr<&'a ArrayType, Output = ArrayType>
        + ops::BitXor<&'a ArrayType, Output = ArrayType>,
    for<'a> &'a ArrayType: ops::BitAnd<ArrayType, Output = ArrayType>
        + ops::BitOr<ArrayType, Output = ArrayType>
        + ops::BitXor<ArrayType, Output = ArrayType>,
    for<'a, 'b> &'a ArrayType: ops::BitAnd<&'b ArrayType, Output = ArrayType>
        + ops::BitOr<&'b ArrayType, Output = ArrayType>
        + ops::BitXor<&'b ArrayType, Output = ArrayType>,
{
    pub grids: [ArrayType; 2],
    pub current_player: u8,
}

impl<ArrayType> State<ArrayType>
where
    ArrayType: bitarray::BitArray,
    for<'a> ArrayType: ops::BitAnd<&'a ArrayType, Output = ArrayType>
        + ops::BitOr<&'a ArrayType, Output = ArrayType>
        + ops::BitXor<&'a ArrayType, Output = ArrayType>,
    for<'a> &'a ArrayType: ops::BitAnd<ArrayType, Output = ArrayType>
        + ops::BitOr<ArrayType, Output = ArrayType>
        + ops::BitXor<ArrayType, Output = ArrayType>,
    for<'a, 'b> &'a ArrayType: ops::BitAnd<&'b ArrayType, Output = ArrayType>
        + ops::BitOr<&'b ArrayType, Output = ArrayType>
        + ops::BitXor<&'b ArrayType, Output = ArrayType>,
{
    pub fn new() -> State<ArrayType> {
        State {
            grids: [ArrayType::zero(); 2],
            current_player: 0,
        }
    }

    pub fn from_indices(
        player1_indices: &[usize],
        player2_indices: &[usize],
        current_player: u8,
    ) -> State<ArrayType> {
        State {
            grids: [
                ArrayType::from_indices(player1_indices),
                ArrayType::from_indices(player2_indices),
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

pub type TicTacToeState = State<bitarray::BitArray9>;
pub type GomokuState = State<bitarray::BitArray225>;

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