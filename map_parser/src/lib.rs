use serde::{Serialize, Deserialize};
use serde_json::{Result};

mod tests;

pub struct TileData {
    pub tile_type: char,
}

impl TileData {
    fn from_type(tile_type: char) -> TileData {
        TileData {
            tile_type
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GameMapFile {
    rows: Vec<String>,
}

pub struct GameMap {
    pub tiles: Vec<Vec<TileData>>
}

impl GameMap {
    pub fn from_str(map: &str) -> Result<GameMap> {
        let file: GameMapFile = serde_json::from_str(map)?;

        Ok(GameMap {
            tiles: file.rows.into_iter().map(|line| line.chars().map(|tile_type| TileData::from_type(tile_type)).collect()).collect()
        })
    }
}
