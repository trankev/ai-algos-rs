use super::ruleset;
use super::state;

pub trait PermutationIteratorTrait<Rules: ruleset::WithPermutableState>
where
    Self: Iterator<Item = Rules::Permutation>,
    Rules::State: state::ComparableState,
{
    fn new(ruleset: &Rules) -> Self;
}
