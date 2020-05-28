use crate::interface::rulesets;

pub enum Request<RuleSet: rulesets::RuleSetTrait> {
    SetState(RuleSet::State),
    IterateSequentially {
        count: usize,
    },
    IterateParallel {
        count: usize,
        expansions_to_do: usize,
        simulations_to_do: usize,
    },
    ListConsiderations,
    Stop,
}
