use bevy::app::Plugin;
use crate::App;
use crate::camera::systems::{follow_player, spawn_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        println!("Camera plugin loaded");
        app
            .add_startup_system(spawn_camera)
            .add_system(follow_player);
    }
}
