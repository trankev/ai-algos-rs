use crate::rulesets;
use crate::rulesets::PlyIteratorTrait;
use crate::rulesets::PlyTrait;
use crate::rulesets::StateTrait;
use rand::rngs;
use rand::seq::IteratorRandom;

pub fn simulate<RuleSet: rulesets::RuleSetTrait>(
    ruleset: &RuleSet,
    state: &RuleSet::State,
    rng: &mut rngs::ThreadRng,
) -> rulesets::Status {
    log::debug!("Simulating state {:?}", state.ascii_representation());
    let mut current_state = state;
    let mut state;
    loop {
        let status = ruleset.status(current_state);
        if let rulesets::Status::Ongoing = status {
            let available_plies = RuleSet::PlyIterator::new(current_state.clone());
            let ply = available_plies.choose(rng).unwrap();
            state = ruleset.play(&current_state, &ply).unwrap();
            log::debug!(
                "Playing {:?}, resulting in state {:?}",
                ply.ascii_representation(),
                state.ascii_representation()
            );
            current_state = &state;
        } else {
            log::debug!("Simulation ended with status {:?}", status);
            return status;
        }
    }
}
