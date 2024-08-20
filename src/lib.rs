pub mod block;

use bevy::{prelude::*, window::WindowMode};
use wasm_bindgen::prelude::*;

use block::*;

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera and light
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // commands.insert_resource(AmbientLight {
    //     color: Color::srgb_u8(255, 0, 0),
    //     brightness: 0.5,
    // });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2.0, 4.0, 6.0),
        ..default()
    });

    // TEMP: Initialize some blocks
    commands.spawn(Block{
        block_id: BlockId("minecraft:black_concrete".to_string()),
        position: Position{x: 0, y: 0, z: 0},
        sprite: PbrBundle {
            mesh: meshes.add(FULL_BLOCK),
            material: materials.add(Color::BLACK),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    });
    commands.spawn(Block{
        block_id: BlockId("minecraft:red_concrete".to_string()),  
        position: Position{x: 1, y: 0, z: 0},
        sprite: PbrBundle {
            mesh: meshes.add(FULL_BLOCK),
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        }
    });
    commands.spawn(Block{
        block_id: BlockId("minecraft:green_concrete".to_string()),
        position: Position{x: 0, y: 1, z: 0},
        sprite: PbrBundle {
            mesh: meshes.add(FULL_BLOCK),
            material: materials.add(Color::srgb_u8(0, 255, 0)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        }
    });
    commands.spawn(Block{
        block_id: BlockId("minecraft:blue_concrete".to_string()), 
        position: Position{x: 0, y: 0, z: 1},
        sprite: PbrBundle {
            mesh: meshes.add(FULL_BLOCK),
            material: materials.add(Color::srgb_u8(0, 0, 255)),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }
    });
}