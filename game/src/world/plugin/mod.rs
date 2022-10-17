use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use crate::world::components::{Position, Tile, TileType, TileTypes};
use crate::world::systems::setup_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.register_inspectable::<TileTypes>();

        app
            .add_startup_system(setup_world)
            .register_type::<TileType>()
            .register_type::<Position>();
    }
}
