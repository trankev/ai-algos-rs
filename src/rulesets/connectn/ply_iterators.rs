use crate::interface::rulesets;
use crate::rulesets::connectn;
use crate::rulesets::connectn::variants;
use std::marker;

pub struct PlyIterator<Variant: variants::BaseVariant> {
    current_index: usize,
    variant: marker::PhantomData<Variant>,
}

impl<Variant: variants::BaseVariant> rulesets::PlyIteratorTrait<connectn::RuleSet<Variant>>
    for PlyIterator<Variant>
{
    fn new(
        _ruleset: &connectn::RuleSet<Variant>,
        _state: &connectn::State<Variant>,
    ) -> PlyIterator<Variant> {
        PlyIterator::<Variant> {
            current_index: 0,
            variant: marker::PhantomData,
        }
    }

    fn iterate(
        &mut self,
        _ruleset: &connectn::RuleSet<Variant>,
        state: &connectn::State<Variant>,
    ) -> Option<connectn::Ply<Variant>> {
        loop {
            if self.current_index >= Variant::CELL_COUNT {
                return None;
            }
            if state.is_empty(self.current_index) {
                break;
            }
            self.current_index += 1;
        }
        let to_return = self.current_index;
        self.current_index += 1;
        Some(connectn::Ply::<Variant>::new(to_return as u8))
    }
}

#[cfg(test)]
mod tests {
    use super::super::variants;
    use super::*;
    use crate::interface::rulesets::PlyIteratorTrait;
    use crate::rulesets::connectn;
    use std::collections;

    #[test]
    fn test_iterate() {
        let ruleset = connectn::TicTacToe::new();
        let state = connectn::TicTacToeState::from_indices(&[4, 1], &[6, 7], 0);
        let mut iterator =
            <connectn::TicTacToe as rulesets::RuleSetTrait>::PlyIterator::new(&ruleset, &state);
        let expected: collections::HashSet<connectn::Ply<variants::TicTacToe>> = [
            connectn::Ply::new(0),
            connectn::Ply::new(2),
            connectn::Ply::new(3),
            connectn::Ply::new(5),
            connectn::Ply::new(8),
        ]
        .iter()
        .cloned()
        .collect();
        let mut result = collections::HashSet::new();
        while let Some(ply) = iterator.iterate(&ruleset, &state) {
            result.insert(ply);
        }
        assert_eq!(result, expected);
    }
}
