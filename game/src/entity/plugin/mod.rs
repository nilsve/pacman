use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use crate::entity::components::EntityDirections;
use crate::entity::systems::{change_entity_sprite_direction, move_entity};

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        println!("Entity plugin loaded");

        #[cfg(debug_assertions)]
        app.register_inspectable::<EntityDirections>();

        app
            .add_system(change_entity_sprite_direction)
            .add_system(move_entity);
    }
}
