use bevy::utils::HashSet;
use map_parser::GameMap;
use crate::world::components::Position;

mod systems;
pub mod plugin;
pub mod components;

pub struct Map {
    pub map: GameMap,
    pub walkable_positions: HashSet<Position>,
    pub loaded: bool,
}

impl Map {
    pub(crate) fn new(map_data: String) -> Self {

        Map {
            map: GameMap::from_str(&map_data).expect("Map data not readable"),
            walkable_positions: HashSet::new(),
            loaded: false,
        }
    }
}
