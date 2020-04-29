use crate::rulesets;
use crate::rulesets::gomoku;
use std::rc;

pub struct SequentialPlyIterator {
    state: rc::Rc<gomoku::State>,
    current_index: usize,
}

impl rulesets::PlyIterator<gomoku::RuleSet> for SequentialPlyIterator {
    fn new(state: rc::Rc<gomoku::State>) -> SequentialPlyIterator {
        SequentialPlyIterator {
            state,
            current_index: 0,
        }
    }
}

impl Iterator for SequentialPlyIterator {
    type Item = gomoku::Ply;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_index >= gomoku::constants::CELL_COUNT {
                return None;
            }
            if self.state.is_empty(self.current_index) {
                break;
            }
            self.current_index += 1;
        }
        let to_return = self.current_index;
        self.current_index += 1;
        Some(gomoku::Ply {
            index: to_return as u8,
        })
    }
}
