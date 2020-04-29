use super::constants;
use super::plies;
use super::state;
use crate::rulesets;
use crate::utils::bitarray::BitArray;
use crate::utils::grids::strips;

pub struct RuleSet {
    strips: Vec<constants::ArrayType>,
}

impl RuleSet {
    pub fn new() -> RuleSet {
        let strips = strips::CellRuns::new(
            vec![constants::GRID_SIZE, constants::GRID_SIZE],
            constants::RUN_SIZE,
        )
        .map(|indices| constants::ArrayType::from_indices(&indices))
        .collect::<Vec<_>>();
        RuleSet { strips }
    }
}

impl rulesets::BaseRuleSet for RuleSet {
    type State = state::State;
    type Ply = plies::Ply;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

    fn play(
        &self,
        state: &Self::State,
        ply: &Self::Ply,
    ) -> Result<Self::State, rulesets::PlayError> {
        let mut result = (*state).clone();
        if let Err(error) = result.play(ply) {
            return Err(error);
        }
        Ok(result)
    }

    fn status(&self, state: &Self::State) -> rulesets::Status {
        let mut ongoing = false;
        let zero = constants::ArrayType::zero();
        for strip in &self.strips {
            for player in 0u8..2 {
                if (&state.grids[player as usize] & strip) == *strip {
                    return rulesets::Status::Win { player };
                }
                if (&state.grids[player as usize] & strip) == zero {
                    ongoing = true;
                }
            }
        }
        if ongoing {
            rulesets::Status::Ongoing
        } else {
            rulesets::Status::Draw
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::plies;
    use super::RuleSet;
    use crate::rulesets::BaseRuleSet;

    #[test]
    fn test_invalid_move() {
        let game = RuleSet::new();
        let state = game.initial_state();
        let ply = plies::Ply { index: 3 };
        let resulting_state = game.play(&state, &ply).unwrap();
        let result = game.play(&resulting_state, &ply);
        assert!(result.is_err());
    }
}
