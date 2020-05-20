use super::ruleset;

pub trait SymmetryIteratorTrait<RuleSet: ruleset::HasStatesWithSymmetries>
where
    Self: Iterator<Item = RuleSet::Symmetry>,
{
    fn new(ruleset: &RuleSet) -> Self;
}
