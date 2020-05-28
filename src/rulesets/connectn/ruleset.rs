use super::plies;
use super::ply_iterators;
use super::state;
use super::symmetry;
use super::symmetry_iterators;
use super::variants;
use crate::interface::rulesets;
use crate::utils::bitarray;
use crate::utils::grids::strips;
use crate::utils::grids::symmetries;

#[derive(Clone)]
pub struct RuleSet<Variant: variants::BaseVariant> {
    symmetries: symmetries::SymmetryTable,
    strips: Vec<bitarray::BitArray<Variant::ArraySettings>>,
}

impl<Variant: variants::BaseVariant> RuleSet<Variant> {
    pub fn new() -> RuleSet<Variant> {
        let dimensions = vec![Variant::GRID_SIZE, Variant::GRID_SIZE];
        let symmetries = symmetries::SymmetryTable::new(&dimensions);
        let strips = strips::CellRuns::new(dimensions, Variant::RUN_COUNT)
            .map(|indices| bitarray::BitArray::<Variant::ArraySettings>::from_indices(&indices))
            .collect::<Vec<_>>();
        RuleSet { strips, symmetries }
    }

    pub fn grid_symmetry_count(&self) -> usize {
        self.symmetries.permutations.len()
    }
}

impl<Variant: variants::BaseVariant> rulesets::RuleSetTrait for RuleSet<Variant> {
    type State = state::State<Variant>;
    type Ply = plies::Ply<Variant>;
    type PlyIterator = ply_iterators::PlyIterator<Variant>;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

    fn status(&self, state: &Self::State) -> rulesets::Status {
        let mut ongoing = false;
        for strip in &self.strips {
            for player in 0..2 {
                match state.grids[player].compare_with_mask(strip) {
                    bitarray::MaskComparison::Equal => {
                        return rulesets::Status::Win {
                            player: player as u8,
                        }
                    }
                    bitarray::MaskComparison::Zero => ongoing = true,
                    _ => (),
                }
            }
        }
        if ongoing {
            rulesets::Status::Ongoing
        } else {
            rulesets::Status::Draw
        }
    }
}

impl<Variant: variants::BaseVariant> rulesets::Deterministic for RuleSet<Variant> {
    fn play(
        &self,
        state: &Self::State,
        ply: &Self::Ply,
    ) -> Result<Self::State, rulesets::PlayError> {
        let mut result = (*state).clone();
        if let Err(error) = result.play(ply) {
            return Err(error);
        }
        Ok(result)
    }
}

impl<Variant: variants::BaseVariant> rulesets::HasStatesWithSymmetries for RuleSet<Variant> {
    type Symmetry = symmetry::Symmetry;
    type SymmetryIterator = symmetry_iterators::SymmetryIterator;

    fn swap_state(&self, state: &Self::State, permutation: &Self::Symmetry) -> Self::State {
        let permutations =
            &self.symmetries.permutations[permutation.grid_permutation_index as usize];
        state.swap(permutations, permutation.switched_players)
    }

    fn reverse_state(&self, state: &Self::State, permutation: &Self::Symmetry) -> Self::State {
        let permutation_index =
            self.symmetries.reverses[permutation.grid_permutation_index as usize];
        let permutations = &self.symmetries.permutations[permutation_index];
        state.swap(permutations, permutation.switched_players)
    }

    fn swap_ply(&self, ply: &Self::Ply, permutation: &Self::Symmetry) -> Self::Ply {
        let permutation =
            &self.symmetries.permutations[permutation.grid_permutation_index as usize];
        plies::Ply::new(permutation[ply.index as usize] as u8)
    }
}

impl<Variant: variants::BaseVariant> rulesets::EncodableState for RuleSet<Variant> {
    const STATE_SIZE: usize = Variant::CELL_COUNT * 3 + 1;
    const PLY_COUNT: usize = Variant::CELL_COUNT;

    fn encode_state(&self, state: &Self::State) -> Vec<f32> {
        let mut result = vec![0.0; Variant::CELL_COUNT * 3 + 1];
        for index in 0..Variant::CELL_COUNT {
            if state.grids[0].isset(index) {
                result[index] = 1.0;
            } else if state.grids[1].isset(index) {
                result[index + Variant::CELL_COUNT] = 1.0;
            } else {
                result[index + Variant::CELL_COUNT * 2] = 1.0;
            }
        }
        result[Variant::CELL_COUNT * 3] = if state.current_player == 0 { 1.0 } else { 0.0 };
        result
    }

    fn decode_ply(&self, ply_index: usize) -> Self::Ply {
        Self::Ply::new(ply_index as u8)
    }

    fn encode_ply(&self, ply: &Self::Ply) -> usize {
        ply.index as usize
    }
}

#[cfg(test)]
mod tests {
    use super::super::plies;
    use super::*;
    use crate::interface::rulesets::Deterministic;
    use crate::interface::rulesets::HasStatesWithSymmetries;
    use crate::interface::rulesets::RuleSetTrait;
    use crate::interface::rulesets::SymmetryIteratorTrait;
    use std::collections;

    pub type TicTacToe = RuleSet<variants::TicTacToe>;

    #[test]
    fn test_invalid_move() {
        let game = TicTacToe::new();
        let state = game.initial_state();
        let ply = plies::Ply::new(3);
        let resulting_state = game.play(&state, &ply).unwrap();
        let result = game.play(&resulting_state, &ply);
        assert!(result.is_err());
    }

    macro_rules! status_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected) = $value;
                    let game = TicTacToe::new();
                    let state = state::State::from_indices(&p1_indices, &p2_indices, current_player);
                    let status = game.status(&state);
                    assert_eq!(status, expected);
                }
            )*
        }
    }

    status_tests! {
        new_game: ([], [], 0, rulesets::Status::Ongoing),
        ongoing: ([4, 1, 6, 5], [8, 7, 2], 1, rulesets::Status::Ongoing),
        p1_win: ([4, 1, 0, 2], [5, 7, 8], 1, rulesets::Status::Win{player: 0}),
        p2_win: ([1, 2, 5], [4, 0, 8], 0, rulesets::Status::Win{player: 1}),
        draw: ([4, 1, 6, 5], [8, 7, 2, 3], 0, rulesets::Status::Draw),
    }

    #[test]
    fn test_swap_state() {
        let game = TicTacToe::new();
        let state = state::State::from_indices(&[1, 2, 4, 7], &[0, 3, 6], 1);
        let permutations =
            <TicTacToe as rulesets::HasStatesWithSymmetries>::SymmetryIterator::new(&game);
        let mut permutation_set = collections::HashSet::new();
        for permutation in permutations {
            let permuted = game.swap_state(&state, &permutation);
            let reverse = game.reverse_state(&permuted, &permutation);
            permutation_set.insert(permuted);
            assert_eq!(state, reverse);
        }
        assert_eq!(permutation_set.len(), 16);
    }
}
