use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use crate::world::components::{Position, Tile, TileType, TileTypes};
use crate::world::systems::load_tiles;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.register_inspectable::<TileTypes>();

        app
            .add_startup_system(load_tiles)
            // .add_startup_system(setup_walkable_path)
            .register_type::<TileType>()
            .register_type::<Position>();
    }
}
