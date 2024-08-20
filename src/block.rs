use bevy::prelude::*;

#[derive(Component)]
pub struct BlockId(pub String);

#[derive(Component)]
pub struct Position{pub x: i32, pub y: i32, pub z: i32}

#[derive(Bundle)]
pub struct Block {
    pub block_id: BlockId,
    pub position: Position,

    pub sprite: PbrBundle
}

pub const FULL_BLOCK: Cuboid = Cuboid {half_size: Vec3{x: 0.5, y: 0.5, z: 0.5}};

impl Default for Block {
    fn default() -> Self {
        Self {
            block_id: BlockId("minecraft:stone".into()),
            position: Position{x: 0, y: 0, z: 0},
            sprite: Default::default(),
        }
    }
}
