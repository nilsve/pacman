use bevy::prelude::Res;
use crate::entity::components::EntityDirections;
use crate::Map;
use crate::world::components::position::TilePosition;

pub fn can_move_direction(map: &Res<Map>, tile_position: &TilePosition, direction: &EntityDirections) -> bool {
    let next_position = tile_position.apply_direction(direction);
    map.walkable_positions.contains(&next_position)
}

