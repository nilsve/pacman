use bevy::prelude::*;

pub mod position;
pub mod tile;
pub mod candy;

#[derive(Component, Default)]
pub struct World;

#[derive(Bundle, Default)]
pub struct WorldBundle {
    pub world: World,

    #[bundle]
    spatial_bundle: SpatialBundle,
}
