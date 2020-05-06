use crate::rulesets;
use crate::rulesets::StateTrait;

#[derive(Debug)]
pub enum Status {
    NotVisited,
    Terminal {
        global: rulesets::Status,
        player: rulesets::PlayerStatus,
    },
    Visited {
        visits: f32,
        win_rate: f32,
        draw_rate: f32,
    },
}

#[derive(Debug)]
pub struct Node<State: StateTrait> {
    pub state: State,
    pub status: Status,
}

impl<State: StateTrait> Node<State> {
    pub fn new(state: State) -> Node<State> {
        Node {
            state,
            status: Status::NotVisited,
        }
    }

    pub fn is_visited(&self) -> bool {
        match self.status {
            Status::NotVisited => false,
            _ => true,
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

    pub fn new_visited(state: State, visits: usize, wins: usize, draws: usize) -> Node<State> {
        Node {
            state,
            status: Status::Visited {
                visits: visits as f32,
                win_rate: wins as f32 / visits as f32,
                draw_rate: draws as f32 / visits as f32,
            },
        }
    }

    pub fn set_terminal(&mut self, status: rulesets::Status) {
        let player_status = status.player_pov(&self.state.current_player());
        self.status = Status::Terminal {
            global: status,
            player: player_status,
        };
    }

    pub fn add_visit(&mut self) {
        self.status = match self.status {
            Status::Visited {
                visits,
                win_rate,
                draw_rate,
            } => Status::Visited {
                visits: visits + 1.0,
                win_rate: win_rate * visits / (visits + 1.0),
                draw_rate: draw_rate * visits / (visits + 1.0),
            },
            Status::NotVisited => Status::Visited {
                visits: 1.0,
                win_rate: 0.0,
                draw_rate: 0.0,
            },
            _ => return,
        };
    }

    pub fn backpropagate(&mut self, status: &rulesets::Status) {
        self.status = match self.status {
            Status::Visited {
                visits,
                mut win_rate,
                mut draw_rate,
            } => {
                match status.player_pov(&self.state.current_player()) {
                    rulesets::PlayerStatus::Win => win_rate += 1.0 / visits,
                    rulesets::PlayerStatus::Draw => draw_rate += 1.0 / visits,
                    rulesets::PlayerStatus::Loss => return,
                    _ => unreachable!(),
                }
                Status::Visited {
                    visits,
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
            Status::NotVisited => 0.5,
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Win => 0.0,
                rulesets::PlayerStatus::Draw => 0.5,
                rulesets::PlayerStatus::Loss => 1.0,
                _ => unreachable!(),
            },
            Status::Visited {
                visits: _,
                win_rate,
                draw_rate,
            } => 1.0 - win_rate - 0.5 * draw_rate,
        }
    }

    pub fn visits(&self) -> f32 {
        match self.status {
            Status::NotVisited => 0.0,
            Status::Terminal {
                global: _,
                player: _,
            } => 1.0,
            Status::Visited {
                visits,
                win_rate: _,
                draw_rate: _,
            } => visits,
        }
    }

    pub fn win_rate(&self) -> f32 {
        match &self.status {
            Status::NotVisited => -1.0,
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Win => 1.0,
                _ => 0.0,
            },
            Status::Visited {
                visits: _,
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
            Status::NotVisited => -1.0,
            Status::Terminal { global: _, player } => match player {
                rulesets::PlayerStatus::Draw => 1.0,
                _ => 0.0,
            },
            Status::Visited {
                visits: _,
                win_rate: _,
                draw_rate,
            } => *draw_rate,
        }
    }
}
