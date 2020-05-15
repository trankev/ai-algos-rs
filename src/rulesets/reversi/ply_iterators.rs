use super::plies;
use super::ruleset;
use super::state;
use super::variants;
use crate::interface;
use crate::utils::grids::strips;
use std::collections;
use std::marker;
use std::sync;

#[derive(Clone, Copy, Eq, PartialEq)]
enum CellState {
    Empty { index: usize },
    Player(interface::Player),
}

pub struct PlyIterator<Variant: variants::BaseVariant> {
    pub strips: sync::Arc<Vec<strips::Indices>>,
    current_index: isize,
    current_strip: strips::Indices,
    strip_state: (CellState, CellState),
    seen: collections::HashSet<usize>,
    variant: marker::PhantomData<Variant>,
}

impl<Variant: variants::BaseVariant> PlyIterator<Variant> {
    fn iterate_strip(
        &mut self,
        state: &state::State<Variant>,
    ) -> Option<(interface::Player, usize)> {
        for index in self.current_strip.by_ref() {
            let mut cell_state = CellState::Empty { index };
            for player in 0..2 {
                if state.grids[player].isset(index) {
                    cell_state = CellState::Player(player as u8);
                    break;
                }
            }
            if cell_state != self.strip_state.1 {
                let (first_state, second_state) = self.strip_state;
                self.strip_state = (second_state, cell_state);
                match (first_state, second_state, cell_state) {
                    (
                        CellState::Empty { index: empty_index },
                        CellState::Player(_),
                        CellState::Player(current_player),
                    ) => return Some((current_player, empty_index)),
                    (
                        CellState::Player(current_player),
                        CellState::Player(_),
                        CellState::Empty { index: empty_index },
                    ) => return Some((current_player, empty_index)),
                    _ => (),
                }
            }
        }
        None
    }

    fn iterate_grid(
        &mut self,
        state: &state::State<Variant>,
    ) -> Option<(interface::Player, usize)> {
        while let Some((player, index)) = self.iterate_strip(state) {
            return Some((player, index));
        }
        self.current_index += 1;
        if self.current_index as usize >= self.strips.len() {
            return None;
        }
        self.current_strip = self.strips[self.current_index as usize].clone();
        self.current_strip.reset();
        self.strip_state = (
            CellState::Empty {
                index: self.current_strip.start as usize,
            },
            CellState::Empty {
                index: self.current_strip.start as usize,
            },
        );
        self.iterate_grid(state)
    }
}

impl<Variant: variants::BaseVariant> interface::PlyIteratorTrait<ruleset::Reversi<Variant>>
    for PlyIterator<Variant>
{
    fn new(ruleset: &ruleset::Reversi<Variant>, state: &state::State<Variant>) -> Self {
        PlyIterator::<Variant> {
            strips: ruleset.strips.clone(),
            current_strip: strips::Indices::empty(),
            current_index: -1,
            strip_state: (CellState::Empty { index: 0 }, CellState::Empty { index: 0 }),
            seen: collections::HashSet::new(),
            variant: marker::PhantomData,
        }
    }

    fn iterate(
        &mut self,
        _ruleset: &ruleset::Reversi<Variant>,
        state: &state::State<Variant>,
    ) -> Option<plies::Ply> {
        while let Some((player, index)) = self.iterate_grid(state) {
            if player != state.current_player || self.seen.contains(&index) {
                continue;
            }
            self.seen.insert(index);
            return Some(plies::Ply::Place(index));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::instances;
    use super::super::plies;
    use super::super::ruleset;
    use super::*;
    use crate::interface::PlyIteratorTrait;
    use crate::interface::RuleSetTrait;
    use crate::interface::StateTrait;
    use std::collections;

    #[test]
    fn test_initial_state() {
        let ruleset = ruleset::Reversi::<instances::Classic>::new();
        let state = ruleset.initial_state();
        println!("{}", state.ascii_representation());
        let mut iterator = PlyIterator::new(&ruleset, &state);
        let mut result = collections::HashSet::<plies::Ply>::new();
        while let Some(ply) = iterator.iterate(&ruleset, &state) {
            result.insert(ply);
        }
        let expected: collections::HashSet<plies::Ply> = [
            plies::Ply::Place(29),
            plies::Ply::Place(34),
            plies::Ply::Place(20),
            plies::Ply::Place(43),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(result, expected);
    }

    macro_rules! iterator_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected) = $value;
                    let ruleset = ruleset::Reversi::<instances::Classic>::new();
                    let state = state::State::from_indices(&p1_indices, &p2_indices, current_player);
                    let mut iterator = PlyIterator::new(&ruleset, &state);
                    let mut result = Vec::new();
                    while let Some(ply) = iterator.iterate(&ruleset, &state) {
                        result.push(ply);
                    }
                    let mut expected: Vec<plies::Ply> = expected
                        .iter()
                        .map(|index| plies::Ply::Place(*index))
                        .collect();
                    result.sort();
                    expected.sort();
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    iterator_tests! {
        single_in_between: ([2], [1], 0, [0]),
        several_in_between: ([5], [1, 2, 3, 4], 0, [0]),
        duplicates: ([2, 16], [1, 8], 0, [0]),
    }
}
