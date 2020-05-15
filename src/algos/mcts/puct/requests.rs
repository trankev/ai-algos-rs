use crate::interface;

pub enum Request<RuleSet: interface::RuleSetTrait> {
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
