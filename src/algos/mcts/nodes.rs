use crate::interface;
use crate::interface::StateTrait;

#[derive(Debug)]
pub enum Status {
    Terminal {
        global: interface::Status,
        player: interface::PlayerStatus,
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
    pub fn new(state: State, status: interface::Status) -> Node<State> {
        let status = if let interface::Status::Ongoing = status {
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

    pub fn game_status(&self) -> interface::Status {
        match self.status {
            Status::Terminal { global, player: _ } => global,
            _ => interface::Status::Ongoing,
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

    pub fn backpropagate(&mut self, status: &interface::Status) {
        self.status = match self.status {
            Status::Ongoing {
                mut score,
                mut draw_rate,
            } => {
                match status.player_pov(&self.state.current_player()) {
                    interface::PlayerStatus::Loss => score += 1.0 / self.visits,
                    interface::PlayerStatus::Draw => {
                        score += 0.5 / self.visits;
                        draw_rate += 1.0 / self.visits;
                    }
                    interface::PlayerStatus::Win => return,
                    interface::PlayerStatus::Ongoing => unreachable!(),
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
                interface::PlayerStatus::Win => 0.0,
                interface::PlayerStatus::Draw => 0.5,
                interface::PlayerStatus::Loss => 1.0,
                interface::PlayerStatus::Ongoing => unreachable!(),
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
                interface::PlayerStatus::Loss => 1.0,
                _ => 0.0,
            },
            Status::Ongoing { score, draw_rate } => score - 0.5 * draw_rate,
        }
    }

    pub fn draw_rate(&self) -> f32 {
        match &self.status {
            Status::Terminal { global: _, player } => match player {
                interface::PlayerStatus::Draw => 1.0,
                _ => 0.0,
            },
            Status::Ongoing {
                score: _,
                draw_rate,
            } => *draw_rate,
        }
    }
}
