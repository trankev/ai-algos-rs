use super::base;
use crate::rulesets::reversi;

pub struct ConvolutionalReversi {}

impl<Variant: reversi::BaseVariant> base::Implementation<reversi::Reversi<Variant>>
    for ConvolutionalReversi
{
    const STATE_DIMENSIONS: &'static [usize] = &[Variant::GRID_SIZE, Variant::GRID_SIZE];
    const PLY_COUNT: usize = Variant::CELL_COUNT + 1;

    fn encode_state(state: &reversi::State<Variant>) -> Vec<f32> {
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

    fn decode_ply(ply_index: usize) -> reversi::Ply<Variant> {
        if ply_index == Variant::CELL_COUNT {
            return reversi::Ply::Pass;
        }
        reversi::Ply::Place(ply_index)
    }

    fn encode_ply(ply: &reversi::Ply<Variant>) -> usize {
        match *ply {
            reversi::Ply::Place(index) => index,
            reversi::Ply::Pass => Variant::CELL_COUNT,
            reversi::Ply::Unused(_) => unreachable!(),
        }
    }
}
