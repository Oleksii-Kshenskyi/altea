use bevy::prelude::*;

use crate::simulation::config::*;

use super::util::*;

pub struct BevyAgentsPlugin;

#[derive(Component)]
pub struct Agent;

#[derive(Component, Clone, Debug)]
pub struct BoardPosition {
    pub x: u32,
    pub y: u32,
}

impl BoardPosition {
    pub fn from_xy(x: u32, y: u32) -> Self {
        BoardPosition { x, y }
    }
    pub fn from_xy_i(x: i32, y: i32) -> Self {
        BoardPosition {
            x: x as u32,
            y: y as u32,
        }
    }
    pub fn from_xy_f(x: f32, y: f32) -> Self {
        BoardPosition {
            x: x as u32,
            y: y as u32,
        }
    }
    pub fn from_xy_us(x: usize, y: usize) -> Self {
        BoardPosition {
            x: x as u32,
            y: y as u32,
        }
    }

    pub fn safely_move(&self, direction_x: i32, direction_y: i32) -> BoardPosition {
        let (new_x, new_y) = (self.x as i32 + direction_x, self.y as i32 + direction_y);
        let x_safe = new_x >= 0 && new_x < TILE_COUNT as i32;
        let y_safe = new_y >= 0 && new_y < TILE_COUNT as i32;

        if x_safe && y_safe {
            BoardPosition::from_xy(self.x + direction_x as u32, self.y + direction_y as u32)
        } else {
            self.clone()
        }
    }
}

fn spawn_agents(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (board_x, board_y) = (TILE_COUNT.div_ceil(2), TILE_COUNT.div_ceil(2));
    let (x, y) = board_to_xy(board_x, board_y);
    let agent_mesh = meshes.add(Circle::new(TILE_SIZE * 0.2));
    let agent_color = materials.add(Color::srgb(0.5, 0.0, 0.0));
    commands.spawn((
        Mesh2d(agent_mesh),
        MeshMaterial2d(agent_color),
        Transform::from_xyz(x, y, 1.0),
        BoardPosition::from_xy_us(board_x, board_y),
        Agent,
    ));
}

fn randomly_move_agent(
    mut player_transforms: Query<(&mut Transform, &mut BoardPosition), With<Agent>>,
) {
    for (mut transform, mut board_pos) in player_transforms.iter_mut() {
        let (dirx, diry) = get_random_direction();
        *board_pos = board_pos.safely_move(dirx, diry);

        let (new_x, new_y) = board_to_xy_u32(board_pos.x, board_pos.y);
        transform.translation.x = new_x;
        transform.translation.y = new_y;
    }
}

impl Plugin for BevyAgentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_agents)
            .add_systems(FixedUpdate, randomly_move_agent);
    }
}
