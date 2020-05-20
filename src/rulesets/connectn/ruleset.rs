use super::plies;
use super::ply_iterators;
use super::state;
use super::symmetry;
use super::symmetry_iterators;
use super::variants;
use crate::interface;
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

impl<Variant: variants::BaseVariant> interface::RuleSetTrait for RuleSet<Variant> {
    type State = state::State<Variant>;
    type Ply = plies::Ply<Variant>;
    type PlyIterator = ply_iterators::PlyIterator<Variant>;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

    fn status(&self, state: &Self::State) -> interface::Status {
        let mut ongoing = false;
        for strip in &self.strips {
            for player in 0..2 {
                match state.grids[player].compare_with_mask(strip) {
                    bitarray::MaskComparison::Equal => {
                        return interface::Status::Win {
                            player: player as u8,
                        }
                    }
                    bitarray::MaskComparison::Zero => ongoing = true,
                    _ => (),
                }
            }
        }
        if ongoing {
            interface::Status::Ongoing
        } else {
            interface::Status::Draw
        }
    }
}

impl<Variant: variants::BaseVariant> interface::Deterministic for RuleSet<Variant> {
    fn play(
        &self,
        state: &Self::State,
        ply: &Self::Ply,
    ) -> Result<Self::State, interface::PlayError> {
        let mut result = (*state).clone();
        if let Err(error) = result.play(ply) {
            return Err(error);
        }
        Ok(result)
    }
}

impl<Variant: variants::BaseVariant> interface::HasStatesWithSymmetries for RuleSet<Variant> {
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

pub type TicTacToe = RuleSet<variants::TicTacToe>;
pub type Gomoku = RuleSet<variants::Gomoku>;

#[cfg(test)]
mod tests {
    use super::super::plies;
    use super::*;
    use crate::interface::Deterministic;
    use crate::interface::HasStatesWithSymmetries;
    use crate::interface::RuleSetTrait;
    use crate::interface::SymmetryIteratorTrait;
    use std::collections;

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
        new_game: ([], [], 0, interface::Status::Ongoing),
        ongoing: ([4, 1, 6, 5], [8, 7, 2], 1, interface::Status::Ongoing),
        p1_win: ([4, 1, 0, 2], [5, 7, 8], 1, interface::Status::Win{player: 0}),
        p2_win: ([1, 2, 5], [4, 0, 8], 0, interface::Status::Win{player: 1}),
        draw: ([4, 1, 6, 5], [8, 7, 2, 3], 0, interface::Status::Draw),
    }

    #[test]
    fn test_swap_state() {
        let game = TicTacToe::new();
        let state = state::State::from_indices(&[1, 2, 4, 7], &[0, 3, 6], 1);
        let permutations =
            <TicTacToe as interface::HasStatesWithSymmetries>::SymmetryIterator::new(&game);
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
