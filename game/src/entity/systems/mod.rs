use std::f32::consts::PI;
use bevy::prelude::*;
use crate::entity::components::{EntityDirections, CurrentDirection, DirectionChange};
use crate::Map;
use crate::world::components::{EntityPosition, TilePosition};
use crate::world::helpers::can_move_direction;

pub fn change_entity_sprite_direction(
    mut query: Query<(&mut Transform, &CurrentDirection), Changed<CurrentDirection>>
) {
    if let Some((mut transform, direction)) = query.iter_mut().next() {
        if let Some(direction) = &direction.0 {
            transform.rotation = match direction {
                EntityDirections::Up => Quat::from_rotation_z(90.0 * PI / 180.0),
                EntityDirections::Down => Quat::from_rotation_z(-90.0 * PI / 180.0),
                EntityDirections::Left => Quat::from_rotation_y(PI),
                EntityDirections::Right => Quat::default(),
            };
        }
    }
}

pub fn move_entity(
    mut query: Query<(&mut EntityPosition, &mut TilePosition, &mut CurrentDirection, &mut DirectionChange)>,
    map: Res<Map>,
) {
    for (mut position, mut next_position, mut current_direction_ref, mut direction_change) in query.iter_mut() {
        if let Some(direction) = direction_change.0 {
            current_direction_ref.0 = Some(direction);
            direction_change.0 = None;
        }

        if let Some(direction) = current_direction_ref.0 {
            *position = next_position.into_entity_position();

            if !can_move_direction(&map, &next_position, &direction) {
                // Get other direction
                current_direction_ref.0 = find_new_direction(&map, &direction, &mut next_position);
            }

            *next_position = next_position.apply_direction(&current_direction_ref.0.unwrap());
        }
    }
}

pub fn update_entity_positions(
    mut query: Query<(&mut EntityPosition, &CurrentDirection)>,
    time: Res<Time>
) {
    let speed = 2.0 * time.delta_seconds();
    for (mut entity_position, direction) in query.iter_mut() {
        if let Some(direction) = direction.0 {
            match direction {
                EntityDirections::Up => { entity_position.y += speed; }
                EntityDirections::Down => { entity_position.y -= speed; }
                EntityDirections::Left => { entity_position.x -= speed; }
                EntityDirections::Right => { entity_position.x += speed; }
            }
        }
    }
}

fn find_new_direction(map: &Res<Map>, current_direction: &EntityDirections, next_position: &mut Mut<TilePosition>) -> Option<EntityDirections> {
    match current_direction {
        EntityDirections::Up | EntityDirections::Down => [EntityDirections::Left, EntityDirections::Right, EntityDirections::Up, EntityDirections::Down],
        EntityDirections::Left | EntityDirections::Right => [EntityDirections::Up, EntityDirections::Down, EntityDirections::Left, EntityDirections::Right]
    }.iter()
        .find(|direction| {
            can_move_direction(&map, &next_position, &direction)
        }).map(|direction| *direction)
}
