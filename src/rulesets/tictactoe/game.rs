use crate::rulesets;
use super::plies;
use super::state;

pub struct TicTacToe {

}

impl rulesets::RuleSet for TicTacToe {
    type State = state::State;
    type Ply = plies::Ply;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

    fn available_plies(&self, state: &Self::State) -> Vec<Self::Ply> {
        (0..9).filter(
            |&index| state.isempty(index)
        ).map(
            |index| plies::Ply{index}
        ).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::TicTacToe;
    use super::super::plies;
    use crate::rulesets::RuleSet;

    #[test]
    fn test_available_plies_new_game() {
        let game = TicTacToe{};
        let state = game.initial_state();
        let available_plies = game.available_plies(&state);
        let expected: Vec<plies::Ply> = (0..9).map(|index| plies::Ply{index}).collect();
        assert_eq!(available_plies, expected);
    }
}
