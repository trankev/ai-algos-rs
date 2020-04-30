pub struct Edge<Ply> {
    ply: Ply,
}

impl<Ply> Edge<Ply> {
    pub fn new(ply: Ply) -> Edge<Ply> {
        Edge { ply }
    }
}
