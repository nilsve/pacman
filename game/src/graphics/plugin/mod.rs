use bevy::prelude::*;
use crate::graphics::systems::{load_textures, sync_positions};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        println!("Graphics plugin loaded");
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
            .add_system(sync_positions);
    }
}
