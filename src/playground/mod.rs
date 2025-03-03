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

use altea::simulation::config::*;

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
        .insert_resource(ClearColor(Color::srgb(0.15, 0.15, 0.15)))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((Camera2d, Msaa::Sample8));
    let mut main_window = windows.single_mut();
    main_window.set_maximized(true);
}
