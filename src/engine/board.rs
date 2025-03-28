use bevy::prelude::*;

use crate::engine::config::*;
use crate::simulation::config::*;

pub struct BevyBoardPlugin;

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for board_col in 0..TILE_COUNT {
        for board_row in 0..TILE_COUNT {
            let margin = 3.;
            let offset = (TILE_SIZE + margin) * (TILE_COUNT as f32 / 2.) - TILE_SIZE as f32 / 2.;
            let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));
            let tile_color = materials.add(ALTEA_TILE_COLOR);

            let x = board_col as f32 * (TILE_SIZE + margin) - offset;
            let y = board_row as f32 * (TILE_SIZE + margin) - offset;

            commands.spawn((
                Mesh2d(tile_mesh),
                MeshMaterial2d(tile_color),
                Transform::from_xyz(x, y, 0.1),
            ));
        }
    }
}

impl Plugin for BevyBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board);
    }
}
