use crate::rulesets;
use crate::rulesets::connectn;
use crate::rulesets::connectn::variants;

pub struct PlyIterator<Variant: variants::BaseVariant> {
    state: connectn::State<Variant>,
    current_index: usize,
}

impl<Variant: variants::BaseVariant> rulesets::PlyIteratorTrait<connectn::RuleSet<Variant>>
    for PlyIterator<Variant>
{
    fn new(state: connectn::State<Variant>) -> PlyIterator<Variant> {
        PlyIterator::<Variant> {
            state,
            current_index: 0,
        }
    }

    fn current_state(&self) -> &connectn::State<Variant> {
        &self.state
    }
}

impl<Variant: variants::BaseVariant> Iterator for PlyIterator<Variant> {
    type Item = connectn::Ply;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_index >= Variant::CELL_COUNT {
                return None;
            }
            if self.state.is_empty(self.current_index) {
                break;
            }
            self.current_index += 1;
        }
        let to_return = self.current_index;
        self.current_index += 1;
        Some(connectn::Ply {
            index: to_return as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rulesets::connectn;
    use crate::rulesets::PlyIteratorTrait;
    use std::collections;

    #[test]
    fn test_iterate() {
        let state = connectn::TicTacToeState::from_indices(&[4, 1], &[6, 7], 0);
        let iterator = <connectn::TicTacToe as rulesets::RuleSetTrait>::PlyIterator::new(state);
        let expected: collections::HashSet<connectn::Ply> = [
            connectn::Ply { index: 0 },
            connectn::Ply { index: 2 },
            connectn::Ply { index: 3 },
            connectn::Ply { index: 5 },
            connectn::Ply { index: 8 },
        ]
        .iter()
        .cloned()
        .collect();
        let result: collections::HashSet<connectn::Ply> = iterator.collect();
        assert_eq!(result, expected);
    }
}
