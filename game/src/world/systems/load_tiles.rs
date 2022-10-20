use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

use crate::graphics::data::PacmanSheet;
use crate::Map;
use crate::world::components::position::{EntityPosition, TilePosition};
use crate::world::components::tile::{PlayerSpawn, Tile, Tiles, TilesBundle, TileTypes, Wall};
use crate::world::components::World;
use crate::world::events::{TilesLoaded, WorldLoaded};
use crate::world::systems::TileTypeResolver;

pub fn setup_tiles(
    mut commands: Commands,
    mut map: ResMut<Map>,
    world_query: Query<(Entity), With<World>>,
    mut world_loaded: EventReader<WorldLoaded>,
    mut event_map_loaded: EventWriter<TilesLoaded>,
    pacman_textures: Res<PacmanSheet>
) {
    if world_loaded.is_empty() {
        return
    }

    world_loaded.clear();

    println!("Loading tiles");

    let world_id = world_query.single();


    let tiles = commands
        .spawn_bundle(TilesBundle::default())
        .insert(Name::new("Tiles"))
        .with_children(|commands| {
            let mut walkable_positions: HashSet<TilePosition> = HashSet::new();
            map.map.tiles.iter()
                .enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, tile)| {
                    let position = TilePosition {
                        x: x as u32,
                        y: y as u32,
                    };

                    if let Some(tile_type) = tile.get_type() {
                        let mut child = commands.spawn();
                        child
                            .insert(EntityPosition {
                                x: x as f32,
                                y: y as f32,
                            }).insert_bundle(Tile {
                                name: Name::new("Tile"),
                                position,
                            });

                        if tile_type == TileTypes::Wall {
                            child
                                .insert(Wall)
                                .insert_bundle(SpriteSheetBundle {
                                    texture_atlas: pacman_textures.0.clone(),
                                    sprite: TextureAtlasSprite::new(9),
                                    transform: Transform {
                                        scale: Vec3::new(1.1, 1.1, 1.0),
                                        ..default()
                                    },
                                    ..Default::default()
                                });
                        } else if tile_type == TileTypes::PlayerSpawn {
                            child.insert(PlayerSpawn);
                        }
                    } else {
                        walkable_positions.insert(position);
                    }
                })
            });
        }).id();

    commands.entity(world_id).push_children(&[tiles]);

    println!("Send map loaded event");
    event_map_loaded.send(TilesLoaded);
}
