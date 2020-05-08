use crate::rulesets;

pub enum Request<RuleSet: rulesets::RuleSetTrait> {
    SetState(RuleSet::State),
    Iterate { count: usize },
    ListConsiderations,
    Stop,
}
