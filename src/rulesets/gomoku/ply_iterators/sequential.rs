use crate::rulesets;
use crate::rulesets::gomoku;
use crate::rulesets::gomoku::variants;
use crate::utils::bitarray;
use std::ops;
use std::rc;

pub struct SequentialPlyIterator<ArrayType, Variant>
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
    state: rc::Rc<<gomoku::RuleSet<ArrayType, Variant> as rulesets::BaseRuleSet>::State>,
    current_index: usize,
}

impl<ArrayType, Variant> rulesets::PlyIterator<gomoku::RuleSet<ArrayType, Variant>>
    for SequentialPlyIterator<ArrayType, Variant>
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
    fn new(
        state: rc::Rc<<gomoku::RuleSet<ArrayType, Variant> as rulesets::BaseRuleSet>::State>,
    ) -> SequentialPlyIterator<ArrayType, Variant> {
        SequentialPlyIterator {
            state,
            current_index: 0,
        }
    }
}

impl<ArrayType, Variant> Iterator for SequentialPlyIterator<ArrayType, Variant>
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
    type Item = gomoku::Ply;

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
        Some(gomoku::Ply {
            index: to_return as u8,
        })
    }
}

pub type TicTacToePlyIterator = SequentialPlyIterator<bitarray::BitArray9, variants::TicTacToe>;
pub type GomokuPlyIterator = SequentialPlyIterator<bitarray::BitArray225, variants::Gomoku>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rulesets::gomoku;
    use crate::rulesets::PlyIterator;
    use std::collections;
    use std::rc;

    #[test]
    fn test_iterate() {
        let state = rc::Rc::new(gomoku::TicTacToeState::from_indices(&[4, 1], &[6, 7], 0));
        let iterator = TicTacToePlyIterator::new(state);
        let expected: collections::HashSet<gomoku::Ply> = [
            gomoku::Ply { index: 0 },
            gomoku::Ply { index: 2 },
            gomoku::Ply { index: 3 },
            gomoku::Ply { index: 5 },
            gomoku::Ply { index: 8 },
        ]
        .iter()
        .cloned()
        .collect();
        let result: collections::HashSet<gomoku::Ply> = iterator.collect();
        assert_eq!(result, expected);
    }
}
