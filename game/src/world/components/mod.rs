use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};


#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Component, Default)]
pub struct Tiles;

#[derive(Bundle)]
pub struct Tile {
    pub(crate) position: Position,

    pub(crate) tile_type: TileType,

    pub name: Name,
}
#[derive(Inspectable, Debug, Clone, Reflect, Default, Eq, PartialEq, Copy)]
pub enum TileTypes {
    #[default]
    Wall,
    PlayerSpawn,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TileType(pub TileTypes);

