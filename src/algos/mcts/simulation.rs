use crate::rulesets;
use crate::rulesets::PlyIteratorTrait;
use rand::rngs;
use rand::seq::IteratorRandom;
use std::rc;

pub fn simulate<RuleSet: rulesets::RuleSetTrait>(
    ruleset: &RuleSet,
    state: rc::Rc<RuleSet::State>,
    rng: &mut rngs::ThreadRng,
) -> rulesets::Status {
    let mut current_state = state;
    loop {
        let status = ruleset.status(&current_state);
        if let rulesets::Status::Ongoing = status {
            let available_plies = RuleSet::PlyIterator::new(current_state.clone());
            let ply = available_plies.choose(rng).unwrap();
            current_state = rc::Rc::new(ruleset.play(&current_state, &ply).unwrap());
        } else {
            return status;
        }
    }
}
