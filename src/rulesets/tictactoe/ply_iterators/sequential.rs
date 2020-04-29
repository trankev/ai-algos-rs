use crate::rulesets;
use crate::rulesets::tictactoe;
use std::rc;

pub struct SequentialPlyIterator {
    state: rc::Rc<tictactoe::State>,
    current_index: usize,
}

impl rulesets::PlyIterator<tictactoe::RuleSet> for SequentialPlyIterator {
    fn new(state: rc::Rc<tictactoe::State>) -> SequentialPlyIterator {
        SequentialPlyIterator {
            state,
            current_index: 0,
        }
    }
}

impl Iterator for SequentialPlyIterator {
    type Item = tictactoe::Ply;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_index >= 9 {
                return None;
            }
            if self.state.is_empty(self.current_index) {
                break;
            }
            self.current_index += 1;
        }
        let to_return = self.current_index;
        self.current_index += 1;
        Some(tictactoe::Ply {
            index: to_return as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SequentialPlyIterator;
    use crate::rulesets::tictactoe::plies;
    use crate::rulesets::tictactoe::state;
    use crate::rulesets::PlyIterator;
    use std::collections;
    use std::rc;

    #[test]
    fn test_iterate() {
        let state = rc::Rc::new(state::State::from_indices(&[4, 1], &[6, 7], 0));
        let iterator = SequentialPlyIterator::new(state);
        let expected: collections::HashSet<plies::Ply> = [
            plies::Ply { index: 0 },
            plies::Ply { index: 2 },
            plies::Ply { index: 3 },
            plies::Ply { index: 5 },
            plies::Ply { index: 8 },
        ]
        .iter()
        .cloned()
        .collect();
        let result: collections::HashSet<plies::Ply> = iterator.collect();
        assert_eq!(result, expected);
    }
}
