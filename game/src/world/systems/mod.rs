use bevy::prelude::*;
use bevy::utils::HashMap;
use map_parser::TileData;

use crate::graphics::data::PacmanSheet;
use crate::Map;
use crate::world::components::{EntityPosition, Tile, TilePosition, Tiles, TileType, TileTypes};
use crate::world::events::MapLoaded;

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

pub fn load_tiles(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut event_map_loaded: EventWriter<MapLoaded>,
    pacman_textures: Res<PacmanSheet>
) {
    println!("Loading map");

    let parent_id = commands
        .spawn_bundle((
            Tiles::default(),
            Name::new("Tiles"),
        )).insert_bundle(SpatialBundle::visible_identity()).id();

    let mut walkable_positions: HashMap<TilePosition, TilePosition> = HashMap::new();
    map.map.tiles.iter()
        .enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, tile)| {
            let position = TilePosition {
                x: x as u32,
                y: y as u32,
            };

            if let Some(tile_type) = tile.get_type() {
                let child = commands.spawn()
                    .insert(EntityPosition {
                        x: x as f32,
                        y: y as f32,
                    }).id();

                commands.entity(child)
                    .insert_bundle(Tile {
                        name: Name::new("Tile"),
                        position,
                        tile_type: TileType(tile_type),
                    });

                if tile_type == TileTypes::Wall {
                    commands.entity(child)
                        .insert_bundle(SpriteSheetBundle {
                            texture_atlas: pacman_textures.0.clone(),
                            sprite: TextureAtlasSprite::new(9),
                            transform: Transform {
                                scale: Vec3::new(1.1, 1.1, 1.0),
                                ..default()
                            },
                            ..Default::default()
                        });
                }

                commands.entity(parent_id).push_children(&[child]);
            } else {
                walkable_positions.insert(position, position);
            }
        })
    });

    map.walkable_positions = walkable_positions;

    println!("Send map loaded event");
    event_map_loaded.send(MapLoaded);
}
