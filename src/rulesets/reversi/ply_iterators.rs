use super::plies;
use super::ruleset;
use super::state;
use super::variants;
use crate::rulesets;
use crate::utils::grids::strips;
use std::collections;
use std::sync;

#[derive(Clone, Copy, Eq, PartialEq)]
enum CellState {
    Empty { index: usize },
    Player(rulesets::Player),
}

pub struct PlyIterator<Variant: variants::BaseVariant> {
    state: state::State<Variant>,
    pub strips: sync::Arc<Vec<strips::Indices>>,
    current_index: isize,
    current_strip: strips::Indices,
    strip_state: (CellState, CellState),
    seen: collections::HashSet<usize>,
}

impl<Variant: variants::BaseVariant> PlyIterator<Variant> {
    fn iterate_strip(&mut self) -> Option<(rulesets::Player, usize)> {
        for index in self.current_strip.by_ref() {
            let mut cell_state = CellState::Empty { index };
            for player in 0..2 {
                if self.state.grids[player].isset(index) {
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

    fn iterate_grid(&mut self) -> Option<(rulesets::Player, usize)> {
        while let Some((player, index)) = self.iterate_strip() {
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
        self.iterate_grid()
    }
}

impl<Variant: variants::BaseVariant> rulesets::PlyIteratorTrait<ruleset::Reversi<Variant>>
    for PlyIterator<Variant>
{
    fn new(ruleset: &ruleset::Reversi<Variant>, state: state::State<Variant>) -> Self {
        PlyIterator::<Variant> {
            state,
            strips: ruleset.strips.clone(),
            current_strip: strips::Indices::empty(),
            current_index: -1,
            strip_state: (CellState::Empty { index: 0 }, CellState::Empty { index: 0 }),
            seen: collections::HashSet::new(),
        }
    }

    fn current_state(&self) -> &state::State<Variant> {
        &self.state
    }
}

impl<Variant: variants::BaseVariant> Iterator for PlyIterator<Variant> {
    type Item = plies::Ply;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((player, index)) = self.iterate_grid() {
            if player != self.state.current_player || self.seen.contains(&index) {
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
    use crate::rulesets::PlyIteratorTrait;
    use crate::rulesets::RuleSetTrait;
    use crate::rulesets::StateTrait;
    use std::collections;

    #[test]
    fn test_initial_state() {
        let ruleset = ruleset::Reversi::<instances::Classic>::new();
        let state = ruleset.initial_state();
        println!("{}", state.ascii_representation());
        let iterator = PlyIterator::new(&ruleset, state);
        let result: collections::HashSet<plies::Ply> = iterator.collect();
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
                    let iterator = PlyIterator::new(&ruleset, state);
                    let mut result: Vec<plies::Ply> = iterator.collect();
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
