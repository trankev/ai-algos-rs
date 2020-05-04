#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Permutation {
    pub grid_permutation_index: u8,
    pub switched_players: bool,
}
