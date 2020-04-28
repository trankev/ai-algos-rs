#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Symmetry {
    pub destination: Vec<bool>,
    pub permutation: Vec<isize>,
}
