use crate::rulesets::RuleSet;
use super::state;

pub struct TicTacToe {

}

impl RuleSet for TicTacToe {
    type State = state::State;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }
}
