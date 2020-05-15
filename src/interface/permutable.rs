use super::permutation_iterator;
use super::ruleset;

pub trait Permutable: ruleset::RuleSetTrait {
    type Permutation;
    type PermutationIterator: permutation_iterator::PermutationIteratorTrait<Self>;

    fn swap_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State;
    fn reverse_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State;
}
