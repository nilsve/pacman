
use bevy::prelude::*;
use crate::entity::components::{CurrentDirection, DirectionChange, Entity};
use crate::world::components::position::{EntityPosition, TilePosition};

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
