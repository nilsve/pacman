use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_inspector_egui::RegisterInspectable;
use crate::entity::components::EntityDirections;
use crate::entity::systems::{change_entity_sprite_direction, move_entity, update_entity_positions};

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        println!("Entity plugin loaded");

        #[cfg(debug_assertions)]
        app.register_inspectable::<EntityDirections>();

        app
            .add_system(change_entity_sprite_direction)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(move_entity)
            )
            .add_system(update_entity_positions);
    }
}
