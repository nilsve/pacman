use std::f32::consts::PI;
use bevy::prelude::*;
use crate::world::components::Position;
use crate::entity::components::{Entity, EntityDirection};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub entity: Entity,
    pub player: Player,
    pub position: Position,
    pub direction: EntityDirection,

    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub name: Name,
}
