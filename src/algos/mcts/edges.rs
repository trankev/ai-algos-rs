use std::fmt;

#[derive(Debug)]
pub struct Edge<Ply: fmt::Debug> {
    pub ply: Ply,
}

impl<Ply: fmt::Debug> Edge<Ply> {
    pub fn new(ply: Ply) -> Edge<Ply> {
        Edge { ply }
    }
}
