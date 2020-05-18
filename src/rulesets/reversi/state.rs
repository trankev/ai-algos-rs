use super::variants;
use crate::interface;
use crate::utils::bitarray;

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct State<Variant: variants::BaseVariant> {
    pub grids: [bitarray::BitArray<Variant::ArraySettings>; 2],
    pub current_player: interface::Player,
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
}

impl<Variant: variants::BaseVariant> interface::StateTrait for State<Variant> {
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

impl<Variant: variants::BaseVariant> interface::TurnByTurnState for State<Variant> {
    fn current_player(&self) -> interface::Player {
        self.current_player
    }
}
