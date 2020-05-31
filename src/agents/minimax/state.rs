use std::f32;
use std::rc;

#[derive(Debug, PartialEq)]
pub enum State<Ply: Copy> {
    Win,
    Loss,
    Draw,
    Heuristic {
        value: f32,
    },
    TreeSearch {
        value: f32,
        depth: u8,
        ply: Ply,
        next: rc::Rc<State<Ply>>,
    },
    Unset,
}

impl<Ply: Copy> State<Ply> {
    pub fn tree_search(ply: Ply, next: State<Ply>) -> State<Ply> {
        State::TreeSearch {
            value: -next.score(),
            depth: match next {
                State::TreeSearch { depth, .. } => depth + 1,
                _ => 1,
            },
            next: rc::Rc::new(next),
            ply,
        }
    }

    pub fn score(&self) -> f32 {
        match self {
            State::Win => f32::INFINITY,
            State::Loss => f32::NEG_INFINITY,
            State::Draw => 0.0,
            State::Heuristic { value } => *value,
            State::TreeSearch { value, .. } => *value,
            State::Unset => f32::NAN,
        }
    }

    pub fn should_replace(&self, other: &State<Ply>) -> bool {
        match other {
            State::Unset => true,
            _ => -self.score() > other.score(),
        }
    }

    pub fn plies(&self) -> Vec<Ply> {
        let mut result: Vec<Ply> = Vec::new();
        self.list_plies(&mut result);
        result
    }

    fn list_plies(&self, list: &mut Vec<Ply>) {
        if let State::TreeSearch { ply, next, .. } = self {
            list.push(*ply);
            next.list_plies(list);
        }
    }
}
