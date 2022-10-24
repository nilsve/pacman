
use bevy::prelude::*;
use crate::entity::components::{CurrentDirection, DirectionChange};
use crate::world::components::position::{EntityPosition, TilePosition};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Score(pub usize);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: EntityPosition,
    pub next_tile_position: TilePosition,
    pub score: Score,
    pub direction: CurrentDirection,
    pub next_direction: DirectionChange,

    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub name: Name,
}
