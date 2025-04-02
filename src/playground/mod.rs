use bevy::{prelude::*, window::WindowResolution};

use altea::engine::agents::BevyAgentsPlugin;
use altea::engine::board::BevyBoardPlugin;
use altea::engine::config::*;
use altea::simulation::config::*;

fn setup_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    commands.spawn((Camera2d, Msaa::Sample8));
    let mut main_window = windows.single_mut();
    main_window.set_maximized(true);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Altea Playground".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .add_systems(Startup, setup_camera)
        .add_plugins(BevyBoardPlugin)
        .add_plugins(BevyAgentsPlugin)
        .insert_resource(ClearColor(ALTEA_BACKGROUND_COLOR))
        .run();
}
