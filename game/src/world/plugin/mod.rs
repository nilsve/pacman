use bevy::prelude::*;
use crate::world::components::candy::Candy;
use crate::world::components::position::{EntityPosition, TilePosition};

use crate::world::events::{TilesLoaded, WorldLoaded};
use crate::world::systems::load_tiles::setup_tiles;
use crate::world::systems::setup_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WorldLoaded>()
            .add_event::<TilesLoaded>()
            .add_startup_system(setup_world)
            .add_system(setup_tiles)
            .register_type::<Candy>()
            .register_type::<EntityPosition>()
            .register_type::<TilePosition>();
    }
}
