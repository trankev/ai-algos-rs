use crate::rulesets;
use crate::rulesets::ninarow;
use crate::rulesets::ninarow::variants;
use crate::utils::bitarray;
use std::ops;
use std::rc;

pub struct PlyIterator<ArrayType, Variant>
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
    state: rc::Rc<<ninarow::RuleSet<ArrayType, Variant> as rulesets::BaseRuleSet>::State>,
    current_index: usize,
}

impl<ArrayType, Variant> rulesets::PlyIteratorTrait<ninarow::RuleSet<ArrayType, Variant>>
    for PlyIterator<ArrayType, Variant>
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
        state: rc::Rc<<ninarow::RuleSet<ArrayType, Variant> as rulesets::BaseRuleSet>::State>,
    ) -> PlyIterator<ArrayType, Variant> {
        PlyIterator {
            state,
            current_index: 0,
        }
    }
}

impl<ArrayType, Variant> Iterator for PlyIterator<ArrayType, Variant>
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
    type Item = ninarow::Ply;

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
        Some(ninarow::Ply {
            index: to_return as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rulesets::ninarow;
    use crate::rulesets::PlyIteratorTrait;
    use std::collections;
    use std::rc;

    #[test]
    fn test_iterate() {
        let state = rc::Rc::new(ninarow::TicTacToeState::from_indices(&[4, 1], &[6, 7], 0));
        let iterator = <ninarow::TicTacToe as rulesets::BaseRuleSet>::PlyIterator::new(state);
        let expected: collections::HashSet<ninarow::Ply> = [
            ninarow::Ply { index: 0 },
            ninarow::Ply { index: 2 },
            ninarow::Ply { index: 3 },
            ninarow::Ply { index: 5 },
            ninarow::Ply { index: 8 },
        ]
        .iter()
        .cloned()
        .collect();
        let result: collections::HashSet<ninarow::Ply> = iterator.collect();
        assert_eq!(result, expected);
    }
}
