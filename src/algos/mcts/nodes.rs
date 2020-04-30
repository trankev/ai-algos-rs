use std::rc;

pub struct Node<State> {
    pub state: rc::Rc<State>,
    pub visits: f32,
    pub wins: f32,
    pub draws: f32,
}

impl<State> Node<State> {
    pub fn new(state: rc::Rc<State>) -> Node<State> {
        Node {
            state,
            visits: 0.0,
            wins: 0.0,
            draws: 0.0,
        }
    }
}
