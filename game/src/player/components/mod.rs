
use bevy::prelude::*;
use crate::world::components::{EntityPosition, TilePosition};
use crate::entity::components::{Entity, CurrentDirection, DirectionChange};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub entity: Entity,
    pub player: Player,
    pub position: EntityPosition,
    pub next_tile_position: TilePosition,
    pub direction: CurrentDirection,
    pub next_direction: DirectionChange,

    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub name: Name,
}
