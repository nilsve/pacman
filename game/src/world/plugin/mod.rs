use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use crate::world::components::{EntityPosition, TileType, TileTypes};
use crate::world::systems::load_tiles;

use iyes_loopless::prelude::*;
use crate::world::events::MapLoaded;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.register_inspectable::<TileTypes>();

        app
            .add_event::<MapLoaded>()
            .add_startup_system(load_tiles)
            // .add_startup_system(setup_candy)
            .register_type::<TileType>()
            .register_type::<EntityPosition>();
    }
}
