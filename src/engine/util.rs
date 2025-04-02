use rand::{self, seq::IndexedRandom};

use crate::simulation::config::*;

pub fn board_to_xy(board_row: usize, board_col: usize) -> (f32, f32) {
    let offset = (TILE_SIZE + TILE_MARGIN) * (TILE_COUNT as f32 / 2.) - TILE_SIZE / 2.;
    let x = board_col as f32 * (TILE_SIZE + TILE_MARGIN) - offset;
    let y = board_row as f32 * (TILE_SIZE + TILE_MARGIN) - offset;
    (x, -y)
}
pub fn board_to_xy_u32(board_row: u32, board_col: u32) -> (f32, f32) {
    board_to_xy(board_row as usize, board_col as usize)
}

// NOTE: This can return (0, 0). In that case, if this is used as a direction for the next agent move, this will effectively act as an idle turn.
pub fn get_random_direction() -> (i32, i32) {
    let mut rng = rand::rng();
    let choices = [-1, 0, 1];
    let x = *choices.choose(&mut rng).unwrap();
    let y = *choices.choose(&mut rng).unwrap();

    (x, y)
}
