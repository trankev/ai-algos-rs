use crate::rulesets;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ply {
    pub index: u8,
}

impl rulesets::PlyTrait for Ply {}
