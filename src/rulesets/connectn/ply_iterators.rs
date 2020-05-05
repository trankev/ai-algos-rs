use crate::rulesets;
use crate::rulesets::connectn;
use crate::rulesets::connectn::variants;
use crate::utils::bitarray;
use std::marker;
use std::ops;

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
    state: connectn::State<ArrayType>,
    current_index: usize,
    variant: marker::PhantomData<Variant>,
}

impl<ArrayType, Variant> rulesets::PlyIteratorTrait<connectn::RuleSet<ArrayType, Variant>>
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
    fn new(state: connectn::State<ArrayType>) -> PlyIterator<ArrayType, Variant> {
        PlyIterator::<ArrayType, Variant> {
            state,
            current_index: 0,
            variant: marker::PhantomData,
        }
    }

    fn current_state(&self) -> &connectn::State<ArrayType> {
        &self.state
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
