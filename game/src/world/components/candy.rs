use bevy::prelude::*;

use crate::world::components::position::TilePosition;

#[derive(Component, Default)]
pub struct Treats;

#[derive(Bundle, Default)]
pub struct TreatsBundle {
    pub world: Treats,

    #[bundle]
    spatial_bundle: SpatialBundle,
}


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Candy;

#[derive(Bundle, Default)]
pub struct CandyBundle {
    pub(crate) candy: Candy,
    pub(crate) tile_position: TilePosition,

    #[bundle]
    pub(crate) sprite_sheet_bundle: SpriteSheetBundle,
}
