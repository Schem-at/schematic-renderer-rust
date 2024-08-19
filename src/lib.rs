use bevy::{prelude::*, window::WindowMode};
use wasm_bindgen::prelude::*;

// Entry point for the WebAssembly application
#[wasm_bindgen]
pub fn run() {
    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#renderCanvas".into()),
            mode: WindowMode::Fullscreen, // Ensures that it fills the canvas without resizing
            ..default()
        }),
        ..default()
    });
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(plugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // TEMP: Red rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(200.0, 100.0)), // width, height
            ..Default::default()
        },
        ..Default::default()
    });
}