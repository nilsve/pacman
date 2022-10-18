use std::ops::Add;
use bevy::prelude::*;

use crate::player::components::Player;

pub fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..default()
        },
        ..default()
    };
    commands.spawn_bundle(camera);
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>
) {
    if let Some(player) = player_query.iter().next() {
        let speed = 2.0 * time.delta_seconds();

        let mut camera = camera_query.single_mut();
        let movement = Vec3::new(
            (player.translation().x - camera.translation.x) * speed,
            (player.translation().y - camera.translation.y) * speed,
            0.0
        );

        camera.translation = camera.translation.add(movement);
    }
}
