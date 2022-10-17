use bevy::prelude::*;
use bevy::window::CursorIcon::Default;
use bevy_tiled_camera::TiledCameraBundle;
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
    player_query: Query<&GlobalTransform, With<Player>>
) {
    if let Some(player) = player_query.iter().next() {
        let mut camera = camera_query.single_mut();
        camera.translation = player.translation();
    }
}
