use std::f32::consts::PI;
use bevy::prelude::*;
use crate::entity::components::{EntityDirections, EntityDirection};
use crate::world::components::Position;

pub fn change_entity_sprite_direction(
    mut query: Query<(&mut Transform, &EntityDirection), Changed<EntityDirection>>
) {
    if let Some((mut transform, direction)) = query.iter_mut().next() {
        println!("Changed direction");

        transform.rotation =  match direction.0 {
            EntityDirections::Up => Quat::from_rotation_z(90.0 * PI / 180.0),
            EntityDirections::Down => Quat::from_rotation_z(-90.0 * PI / 180.0),
            EntityDirections::Left => Quat::from_rotation_y(PI),
            EntityDirections::Right => Quat::default(),
        };
    }
}

pub fn move_entity(
    mut query: Query<(&mut Position, &EntityDirection)>,
    time: Res<Time>,
) {
    let speed = 2.0 * time.delta_seconds();
    for (mut position, direction) in query.iter_mut() {
        match direction.0 {
            EntityDirections::Up => { position.y += speed },
            EntityDirections::Down => { position.y -= speed },
            EntityDirections::Left => { position.x -= speed },
            EntityDirections::Right => { position.x += speed },
        }
    }
}
