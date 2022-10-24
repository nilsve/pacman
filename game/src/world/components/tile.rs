use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use crate::world::components::position::TilePosition;

#[derive(Component, Default)]
pub struct Tiles;

#[derive(Bundle, Default)]
pub struct TilesBundle {
    pub tiles: Tiles,

    #[bundle]
    spatial_bundle: SpatialBundle,
}

#[derive(Bundle)]
pub struct Tile {
    pub(crate) position: TilePosition,

    pub name: Name,
}

#[derive(Eq, PartialEq)]
pub enum TileTypes {
    Wall,
    Candy,
    PlayerSpawn,
}

#[derive(Component, Default)]
pub struct Wall;

#[derive(Component, Default)]
pub struct PlayerSpawn;
