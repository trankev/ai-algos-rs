use super::permutable;

pub trait PermutationIteratorTrait<Rules: permutable::Permutable>:
    Iterator<Item = Rules::Permutation>
{
    fn new(ruleset: &Rules) -> Self;
}
