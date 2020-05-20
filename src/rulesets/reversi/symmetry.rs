#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Symmetry {
    pub grid_symmetry_index: u8,
    pub switched_players: bool,
}
