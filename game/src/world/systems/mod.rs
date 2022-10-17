use bevy::prelude::*;
use map_parser::TileData;

use crate::graphics::data::PacmanSheet;
use crate::Map;
use crate::world::components::{Position, Tile, Tiles, TileType, TileTypes};

trait TileTypeResolver {
    fn get_type(&self) -> Option<TileTypes>;
}

impl TileTypeResolver for TileData {
    fn get_type(&self) -> Option<TileTypes> {
        match self.tile_type {
            'X' => Some(TileTypes::Wall),
            'P' => Some(TileTypes::PlayerSpawn),
            _ => None,
        }
    }
}

pub fn setup_world(
    mut commands: Commands,
    mut map: ResMut<Map>,
    pacman_textures: Res<PacmanSheet>,
) {
    if !map.loaded {
        println!("Loading map");

        let parent_id = commands
            .spawn_bundle((
                Tiles::default(),
                Name::new("Tiles"),
            )).insert_bundle(SpatialBundle::visible_identity()).id();

        map.map.tiles.iter()
            .enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, tile)| {
                if let Some(tile_type) = tile.get_type() {

                    let child = commands.spawn()
                        .insert(Position {
                            x: x as f32,
                            y: y as f32,
                        }).id();

                    commands.entity(child)
                        .insert_bundle(Tile {
                            name: Name::new("Tile"),
                            position: Position {
                                x: x as f32,
                                y: y as f32,
                            },
                            tile_type: TileType(tile_type),
                        });

                    if tile_type == TileTypes::Wall {
                        commands.entity(child)
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: pacman_textures.0.clone(),
                                sprite: TextureAtlasSprite::new(9),
                                ..Default::default()
                            });
                    }

                    commands.entity(parent_id).push_children(&[child]);
                }
            })
        });

        println!("Map loaded");

        map.loaded = true;
    }
}
