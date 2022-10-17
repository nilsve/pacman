use std::f32::consts::PI;
use std::ops::Add;
use bevy::prelude::*;
use crate::entity::components::{EntityDirections, EntityDirection};
use crate::graphics::data::PacmanSheet;
use crate::Map;
use crate::player::components::{Player, PlayerBundle};
use crate::world::components::{Position, Tile, TileType, TileTypes};

pub fn create_player(
    mut commands: Commands,
    pacman_sheet: Res<PacmanSheet>,
    map: Res<Map>,
    query: Query<(&Position, &TileType)>
) {
    if !map.loaded {
        return
    }
    println!("Create player");

    let (position, _) = query.iter()
        .find(|(_, tile_type)| tile_type.0 == TileTypes::PlayerSpawn)
        .unwrap();

    commands.spawn_bundle(PlayerBundle {
        name: Name::new("Player"),
        position: *position,
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: pacman_sheet.0.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        ..default()
    });
}

pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut EntityDirection, With<Player>>
) {
    let mut next_direction = None;
    if keyboard_input.pressed(KeyCode::W) {
        next_direction = Some(EntityDirections::Up);
    } else if keyboard_input.pressed(KeyCode::A) {
        next_direction = Some(EntityDirections::Left);
    } else if keyboard_input.pressed(KeyCode::S) {
        next_direction = Some(EntityDirections::Down);
    } else if keyboard_input.pressed(KeyCode::D) {
        next_direction = Some(EntityDirections::Right);
    }

    if let Some(next) = next_direction {
        query.single_mut().0 = next;
    }
}


