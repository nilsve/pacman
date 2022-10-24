pub mod load_tiles;

use bevy::prelude::*;
use map_parser::TileData;
use crate::world::components::tile::TileTypes;
use crate::world::components::{World, WorldBundle};
use crate::world::events::WorldLoaded;

trait TileTypeResolver {
    fn get_type(&self) -> Option<TileTypes>;
}

impl TileTypeResolver for TileData {
    fn get_type(&self) -> Option<TileTypes> {
        match self.tile_type {
            'X' => Some(TileTypes::Wall),
            'P' => Some(TileTypes::PlayerSpawn),
            'C' => Some(TileTypes::Candy),
            _ => None,
        }
    }
}

pub fn setup_world(
    mut commands: Commands,
    mut event_emitter: EventWriter<WorldLoaded>
) {
    commands.spawn().insert_bundle(WorldBundle::default()).insert(Name::new("World"));
    event_emitter.send(WorldLoaded);
}
