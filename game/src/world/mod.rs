use map_parser::GameMap;

mod systems;
pub mod plugin;
pub mod components;

pub struct Map {
    pub map: GameMap,
    pub loaded: bool,
}

impl Map {
    pub(crate) fn new(map_data: String) -> Self {

        Map {
            map: GameMap::from_str(&map_data).expect("Map data not readable"),
            loaded: false,
        }
    }
}
