use bevy::utils::{HashMap, HashSet};
use components::position::{EntityPosition, TilePosition};
use map_parser::GameMap;

pub mod systems;
pub mod plugin;
pub mod components;
pub mod events;
pub mod helpers;

pub struct Map {
    pub map: GameMap,
    pub walkable_positions: HashSet<TilePosition>,
}

impl Map {
    pub(crate) fn new(map_data: String) -> Self {
        Map {
            map: GameMap::load_from_string(&map_data).expect("Map data not readable"),
            walkable_positions: HashSet::new(),
        }
    }
}
