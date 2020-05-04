use crate::rulesets;
use std::rc;

pub struct PlyAndState<RuleSet: rulesets::RuleSetTrait> {
    pub ply: RuleSet::Ply,
    pub state: rc::Rc<RuleSet::State>,
}
