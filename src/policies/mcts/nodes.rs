use crate::interface::rulesets;
use crate::interface::rulesets::StateTrait;

#[derive(Debug)]
pub enum Status {
    Terminal { status: rulesets::Status },
    Ongoing { score: f32, draw_rate: f32 },
}

#[derive(Debug)]
pub struct Node<State: StateTrait> {
    pub state: State,
    pub status: Status,
    pub visits: f32,
    pub expanding: bool,
    pub current_player: rulesets::Player,
}

impl<State: rulesets::StateTrait> Node<State> {
    pub fn new(
        state: State,
        status: rulesets::Status,
        current_player: rulesets::Player,
    ) -> Node<State> {
        let status = if let rulesets::Status::Ongoing = status {
            Status::Ongoing {
                score: 0.0,
                draw_rate: 0.0,
            }
        } else {
            Status::Terminal { status }
        };
        Node {
            state,
            status,
            visits: 0.0,
            expanding: false,
            current_player,
        }
    }

    pub fn is_visited(&self) -> bool {
        match self.status {
            Status::Terminal { .. } => true,
            _ => self.visits > 0.0,
        }
    }

    pub fn game_status(&self) -> rulesets::Status {
        match self.status {
            Status::Terminal { status } => status,
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

    pub fn new_visited(
        state: State,
        visits: usize,
        wins: usize,
        draws: usize,
        current_player: rulesets::Player,
    ) -> Node<State> {
        Node {
            state,
            status: Status::Ongoing {
                score: (visits as f32 - wins as f32 - 0.5 * draws as f32) / visits as f32,
                draw_rate: draws as f32 / visits as f32,
            },
            visits: visits as f32,
            expanding: false,
            current_player,
        }
    }

    pub fn backpropagate(&mut self, status: rulesets::Status) {
        self.status = match self.status {
            Status::Ongoing {
                mut score,
                mut draw_rate,
            } => {
                match status.player_pov(self.current_player) {
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
            Status::Terminal { .. } => return,
        };
    }

    pub fn score(&self) -> f32 {
        match &self.status {
            Status::Terminal { status } => match status.player_pov(self.current_player) {
                rulesets::PlayerStatus::Win => 0.0,
                rulesets::PlayerStatus::Draw => 0.5,
                rulesets::PlayerStatus::Loss => 1.0,
                rulesets::PlayerStatus::Ongoing => unreachable!(),
            },
            Status::Ongoing { score, .. } => *score,
        }
    }

    pub fn win_rate(&self) -> f32 {
        match &self.status {
            Status::Terminal { status } => match status.player_pov(self.current_player) {
                rulesets::PlayerStatus::Loss => 1.0,
                _ => 0.0,
            },
            Status::Ongoing { score, draw_rate } => score - 0.5 * draw_rate,
        }
    }

    pub fn draw_rate(&self) -> f32 {
        match &self.status {
            Status::Terminal { status } => match status.player_pov(self.current_player) {
                rulesets::PlayerStatus::Draw => 1.0,
                _ => 0.0,
            },
            Status::Ongoing { draw_rate, .. } => *draw_rate,
        }
    }
}
