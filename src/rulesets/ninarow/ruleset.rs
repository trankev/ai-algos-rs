use super::permutation;
use super::permutation_iterators;
use super::plies;
use super::ply_iterators;
use super::state;
use super::variants;
use crate::rulesets;
use crate::utils::bitarray;
use crate::utils::grids::strips;
use crate::utils::grids::symmetries;
use std::marker;
use std::ops;

pub struct RuleSet<ArrayType, Variant>
where
    Variant: variants::BaseVariant,
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
    variant: marker::PhantomData<Variant>,
    symmetries: symmetries::SymmetryTable,
    strips: Vec<ArrayType>,
}

impl<ArrayType, Variant> RuleSet<ArrayType, Variant>
where
    Variant: variants::BaseVariant,
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
    pub fn new() -> RuleSet<ArrayType, Variant> {
        let dimensions = vec![Variant::GRID_SIZE, Variant::GRID_SIZE];
        let symmetries = symmetries::SymmetryTable::new(&dimensions);
        let strips = strips::CellRuns::new(dimensions, Variant::RUN_COUNT)
            .map(|indices| ArrayType::from_indices(&indices))
            .collect::<Vec<_>>();
        RuleSet {
            strips,
            symmetries,
            variant: marker::PhantomData,
        }
    }

    pub fn grid_symmetry_count(&self) -> usize {
        self.symmetries.permutations.len()
    }
}

impl<ArrayType, Variant> rulesets::RuleSetTrait for RuleSet<ArrayType, Variant>
where
    Variant: variants::BaseVariant,
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
    type State = state::State<ArrayType>;
    type Ply = plies::Ply;
    type PlyIterator = ply_iterators::PlyIterator<ArrayType, Variant>;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

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

    fn status(&self, state: &Self::State) -> rulesets::Status {
        let mut ongoing = false;
        let zero = ArrayType::zero();
        for strip in &self.strips {
            for player in 0u8..2 {
                if (&state.grids[player as usize] & strip) == *strip {
                    return rulesets::Status::Win { player };
                }
                if (&state.grids[player as usize] & strip) == zero {
                    ongoing = true;
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

impl<ArrayType, Variant> rulesets::Permutable for RuleSet<ArrayType, Variant>
where
    Variant: variants::BaseVariant,
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
    type Permutation = permutation::Permutation;
    type PermutationIterator = permutation_iterators::PermutationIterator;

    fn swap_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State {
        let permutations =
            &self.symmetries.permutations[permutation.grid_permutation_index as usize];
        state.swap(permutations, permutation.switched_players)
    }

    fn reverse_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State {
        let permutation_index =
            self.symmetries.reverses[permutation.grid_permutation_index as usize];
        let permutations = &self.symmetries.permutations[permutation_index];
        state.swap(permutations, permutation.switched_players)
    }
}

pub type TicTacToe = RuleSet<bitarray::BitArray9, variants::TicTacToe>;
pub type Gomoku = RuleSet<bitarray::BitArray225, variants::Gomoku>;

#[cfg(test)]
mod tests {
    use super::super::plies;
    use super::*;
    use crate::rulesets::Permutable;
    use crate::rulesets::PermutationIteratorTrait;
    use crate::rulesets::RuleSetTrait;
    use std::collections;

    #[test]
    fn test_invalid_move() {
        let game = TicTacToe::new();
        let state = game.initial_state();
        let ply = plies::Ply { index: 3 };
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
        let permutations = <TicTacToe as rulesets::Permutable>::PermutationIterator::new(&game);
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
