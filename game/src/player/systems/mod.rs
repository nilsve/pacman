use bevy::prelude::*;
use crate::entity::components::{EntityDirections, DirectionChange};
use crate::graphics::data::PacmanSheet;
use crate::Map;
use crate::player::components::{Player, PlayerBundle};
use crate::world::components::{EntityPosition, TilePosition, TileType, TileTypes};
use crate::world::events::MapLoaded;
use crate::world::helpers::can_move_direction;

pub fn create_player(
    mut commands: Commands,
    pacman_sheet: Res<PacmanSheet>,
    query: Query<(&EntityPosition, &TileType)>,
    event: EventReader<MapLoaded>
) {
    if event.is_empty() {
        return
    }

    event.clear();
    println!("Create player");

    let (position, _) = query.iter()
        .find(|(_, tile_type)| {
            tile_type.0 == TileTypes::PlayerSpawn
        })
        .expect("Player spawn not found");

    commands.spawn_bundle(PlayerBundle {
        name: Name::new("Player"),
        position: *position,
        next_tile_position: position.into(),
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
    mut query: Query<(&mut DirectionChange, &TilePosition), With<Player>>,
    map: Res<Map>
) {
    if query.is_empty() {
        return
    }
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

    if let Some(next_direction) = next_direction {
        let (mut direction, position) = query.single_mut();

        if can_move_direction(&map, position, &next_direction) {
            *direction = DirectionChange(Some(next_direction));
        }
    }
}


