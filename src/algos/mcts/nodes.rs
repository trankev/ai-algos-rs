use crate::rulesets;
use crate::rulesets::StateTrait;

#[derive(Debug)]
pub enum Status {
    Terminal {
        global: rulesets::Status,
        player: rulesets::PlayerStatus,
    },
    Ongoing {
        score: f32,
        draw_rate: f32,
    },
}

#[derive(Debug)]
pub struct Node<State: StateTrait> {
    pub state: State,
    pub status: Status,
    pub visits: f32,
    pub expanding: bool,
}

impl<State: StateTrait> Node<State> {
    pub fn new(state: State, status: rulesets::Status) -> Node<State> {
        let status = if let rulesets::Status::Ongoing = status {
            Status::Ongoing {
                score: 0.0,
                draw_rate: 0.0,
            }
        } else {
            let player_status = status.player_pov(&state.current_player());
            Status::Terminal {
                global: status,
                player: player_status,
            }
        };
        Node {
            state,
            status,
            visits: 0.0,
            expanding: false,
        }
    }

    pub fn is_visited(&self) -> bool {
        match self.status {
            Status::Terminal {
                global: _,
                player: _,
            } => true,
            _ => self.visits > 0.0,
        }
    }

    pub fn game_status(&self) -> rulesets::Status {
        match self.status {
            Status::Terminal { global, player: _ } => global,
            _ => rulesets::Status::Ongoing,
        }
    }

    pub fn add_visit(&mut self) {
        self.visits += 1.0;
        if let Status::Ongoing {
            mut score,
            mut draw_rate,
        } = self.status
        {
            let factor = (self.visits - 1.0) / self.visits;
            score *= factor;
            draw_rate *= factor;
            self.status = Status::Ongoing { score, draw_rate };
        }
    }

    pub fn new_visited(state: State, visits: usize, wins: usize, draws: usize) -> Node<State> {
        Node {
            state,
            status: Status::Ongoing {
                score: (visits as f32 - wins as f32 - 0.5 * draws as f32) / visits as f32,
                draw_rate: draws as f32 / visits as f32,
            },
            visits: visits as f32,
            expanding: false,
        }
    }

    pub fn backpropagate(&mut self, status: &rulesets::Status) {
        self.status = match self.status {
            Status::Ongoing {
                mut score,
                mut draw_rate,
            } => {
                match status.player_pov(&self.state.current_player()) {
                    rulesets::PlayerStatus::Loss => score += 1.0 / self.visits,
                    rulesets::PlayerStatus::Draw => {
                        score += 0.5 / self.visits;
                        draw_rate += 1.0 / self.visits;
                    }
                    rulesets::PlayerStatus::Win => return,
                    rulesets::PlayerStatus::Ongoing => unreachable!(),
                }
                Status::Ongoing { score, draw_rate }
            }
            Status::Terminal {
                global: _,
                player: _,
            } => return,
        };
    }

    pub fn score(&self) -> f32 {
        match &self.status {
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Win => 0.0,
                rulesets::PlayerStatus::Draw => 0.5,
                rulesets::PlayerStatus::Loss => 1.0,
                rulesets::PlayerStatus::Ongoing => unreachable!(),
            },
            Status::Ongoing {
                score,
                draw_rate: _,
            } => *score,
        }
    }

    pub fn win_rate(&self) -> f32 {
        match &self.status {
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Loss => 1.0,
                _ => 0.0,
            },
            Status::Ongoing { score, draw_rate } => score - 0.5 * draw_rate,
        }
    }

    pub fn draw_rate(&self) -> f32 {
        match &self.status {
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Draw => 1.0,
                _ => 0.0,
            },
            Status::Ongoing {
                score: _,
                draw_rate,
            } => *draw_rate,
        }
    }
}
