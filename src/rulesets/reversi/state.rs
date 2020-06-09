use super::variants;
use crate::interface::rulesets;
use crate::utils::bitarray;

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct State<Variant: variants::BaseVariant> {
    pub grids: [bitarray::BitArray<Variant::ArraySettings>; 2],
    pub current_player: rulesets::Player,
}

impl<Variant: variants::BaseVariant> State<Variant> {
    pub fn new() -> State<Variant> {
        State {
            grids: [
                bitarray::BitArray::from_indices(Variant::PLAYER_POSITIONS[0]),
                bitarray::BitArray::from_indices(Variant::PLAYER_POSITIONS[1]),
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
    fn ascii_representation(&self) -> String {
        let mut result = String::new();
        for index in 0..Variant::CELL_COUNT {
            if self.grids[0].isset(index) {
                result.push('X');
            } else if self.grids[1].isset(index) {
                result.push('O');
            } else {
                result.push('.');
            }
            if index % Variant::GRID_SIZE == Variant::GRID_SIZE - 1 {
                result.push('\n');
            }
        }
        format!("{}\nTo play: {}", result, self.current_player)
    }
}
