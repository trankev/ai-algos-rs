use super::ruleset;

pub trait PermutationIteratorTrait<RuleSet: ruleset::WithPermutableState>
where
    Self: Iterator<Item = RuleSet::Permutation>,
{
    fn new(ruleset: &RuleSet) -> Self;
}
