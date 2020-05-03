pub struct PlyConsideration<Ply> {
    pub ply: Ply,
    pub score: f32,
    pub follow_up: Vec<Ply>,
}
