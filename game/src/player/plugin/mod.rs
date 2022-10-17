use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use crate::player::components::Player;
use crate::player::systems::{create_player, handle_input};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<Direction>()
            .add_startup_system(create_player)
            .add_system(handle_input);
    }
}
