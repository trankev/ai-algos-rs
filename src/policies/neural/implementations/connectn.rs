use super::base;
use crate::rulesets::connectn;

pub struct ConvolutionalConnectN {}

impl<Variant: connectn::BaseVariant> base::Implementation<connectn::RuleSet<Variant>>
    for ConvolutionalConnectN
{
    const STATE_DIMENSIONS: &'static [usize] = &[Variant::GRID_SIZE, Variant::GRID_SIZE];
    const PLY_COUNT: usize = Variant::CELL_COUNT;

    fn encode_state(state: &connectn::State<Variant>) -> Vec<f32> {
        let player = state.current_player as usize;
        let opponent = 1 - player;
        let result = (0..Variant::CELL_COUNT)
            .map(|index| {
                if state.grids[player].isset(index) {
                    1.0
                } else if state.grids[opponent].isset(index) {
                    -1.0
                } else {
                    0.0
                }
            })
            .collect();
        result
    }

    fn decode_ply(ply_index: usize) -> connectn::Ply<Variant> {
        connectn::Ply::new(ply_index as u8)
    }

    fn encode_ply(ply: &connectn::Ply<Variant>) -> usize {
        ply.index as usize
    }
}
