use bevy::prelude::Res;
use crate::entity::components::EntityDirections;
use crate::Map;
use crate::world::components::{TilePosition};

pub fn can_move_direction(map: &Res<Map>, entity_position: &TilePosition, direction: &EntityDirections) -> bool {
    let next_position = entity_position.apply_direction(direction);
    map.walkable_positions.contains_key(&next_position)
}

