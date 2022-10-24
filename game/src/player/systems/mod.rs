use bevy::prelude::*;
use crate::entity::components::{DirectionChange, EntityDirections};
use crate::graphics::data::PacmanSheet;
use crate::Map;
use crate::player::components::{Player, PlayerBundle, Score};
use crate::world::components::candy::Candy;
use crate::world::components::position::{TilePosition};
use crate::world::components::tile::PlayerSpawn;
use crate::world::events::TilesLoaded;
use crate::world::helpers::can_move_direction;

pub fn create_player(
    mut commands: Commands,
    pacman_sheet: Res<PacmanSheet>,
    player_spawn_query: Query<&TilePosition, With<PlayerSpawn>>,
    event: EventReader<TilesLoaded>
) {
    if event.is_empty() {
        return
    }

    if let Some(position) = player_spawn_query.iter().next() {
        event.clear();
        println!("Create player");

        commands.spawn_bundle(PlayerBundle {
            name: Name::new("Player"),
            position: position.into(),
            next_tile_position: *position,
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: pacman_sheet.0.clone(),
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
            ..default()
        });
    } else {
        println!("Event received but no spawn found");
    }
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

pub fn eat_candy(
    mut commands: Commands,
    candy_query: Query<(Entity, &TilePosition), With<Candy>>,
    mut player_query: Query<(&mut Score, &TilePosition), With<Player>>
) {
    for (mut score, tile_position) in player_query.iter_mut() {
        if let Some((candy_id, _)) = candy_query.iter()
            .find(|(_, candy_position)| *candy_position == tile_position) {
            score.0 += 1;
            commands.entity(candy_id).despawn();
        }
    }
}

