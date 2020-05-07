use crate::rulesets;
use crate::rulesets::StateTrait;

#[derive(Debug)]
pub enum Status {
    Terminal {
        global: rulesets::Status,
        player: rulesets::PlayerStatus,
    },
    Ongoing {
        win_rate: f32,
        draw_rate: f32,
    },
}

#[derive(Debug)]
pub struct Node<State: StateTrait> {
    pub state: State,
    pub status: Status,
    pub visits: f32,
}

impl<State: StateTrait> Node<State> {
    pub fn new(state: State, status: rulesets::Status) -> Node<State> {
        let status = if let rulesets::Status::Ongoing = status {
            Status::Ongoing {
                win_rate: 0.0,
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

    pub fn is_terminal(&self) -> bool {
        match self.status {
            Status::Terminal {
                global: _,
                player: _,
            } => true,
            _ => false,
        }
    }

    pub fn add_visit(&mut self) {
        self.visits += 1.0;
        if let Status::Ongoing {
            mut win_rate,
            mut draw_rate,
        } = self.status
        {
            let factor = (self.visits - 1.0) / self.visits;
            win_rate *= factor;
            draw_rate *= factor;
            self.status = Status::Ongoing {
                win_rate,
                draw_rate,
            };
        }
    }

    pub fn new_visited(state: State, visits: usize, wins: usize, draws: usize) -> Node<State> {
        Node {
            state,
            status: Status::Ongoing {
                win_rate: wins as f32 / visits as f32,
                draw_rate: draws as f32 / visits as f32,
            },
            visits: visits as f32,
        }
    }

    pub fn backpropagate(&mut self, status: &rulesets::Status) {
        self.status = match self.status {
            Status::Ongoing {
                mut win_rate,
                mut draw_rate,
            } => {
                match status.player_pov(&self.state.current_player()) {
                    rulesets::PlayerStatus::Win => win_rate += 1.0 / self.visits,
                    rulesets::PlayerStatus::Draw => draw_rate += 1.0 / self.visits,
                    rulesets::PlayerStatus::Loss => return,
                    _ => unreachable!(),
                }
                Status::Ongoing {
                    win_rate,
                    draw_rate,
                }
            }
            Status::Terminal {
                global: _,
                player: _,
            } => return,
            _ => unreachable!(),
        };
    }

    pub fn score(&self) -> f32 {
        match &self.status {
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Win => 0.0,
                rulesets::PlayerStatus::Draw => 0.5,
                rulesets::PlayerStatus::Loss => 1.0,
                _ => unreachable!(),
            },
            Status::Ongoing {
                win_rate,
                draw_rate,
            } => 1.0 - win_rate - 0.5 * draw_rate,
        }
    }

    pub fn win_rate(&self) -> f32 {
        match &self.status {
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Win => 1.0,
                _ => 0.0,
            },
            Status::Ongoing {
                win_rate,
                draw_rate: _,
            } => *win_rate,
        }
    }

    pub fn global_status(&self) -> rulesets::Status {
        match &self.status {
            Status::Terminal { global, player: _ } => *global,
            _ => unreachable!(),
        }
    }

    pub fn draw_rate(&self) -> f32 {
        match &self.status {
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Draw => 1.0,
                _ => 0.0,
            },
            Status::Ongoing {
                win_rate: _,
                draw_rate,
            } => *draw_rate,
        }
    }
}
