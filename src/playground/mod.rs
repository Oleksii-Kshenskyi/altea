use std::os::windows;

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
    window::{PresentMode, WindowResolution},
};

use altea::engine::board::BevyBoardPlugin;
use altea::engine::config::*;
use altea::simulation::config::*;

fn setup_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
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
        .add_plugins(BevyBoardPlugin)
        .insert_resource(ClearColor(ALTEA_BACKGROUND_COLOR))
        .add_systems(Startup, setup_camera)
        .run();
}
