use bevy::prelude::*;
use bevy::utils::default;
use crate::Map;
use crate::world::components::candy::{CandyBundle, Treats};
use crate::world::components::World;
use crate::world::events::TilesLoaded;

pub fn setup_candy(
    mut commands: Commands,
    world_query: Query<Entity, With<World>>,
    event: EventReader<TilesLoaded>,
    map: Res<Map>
) {
    if event.is_empty() {
        return
    }

    event.clear();

    let world_id = world_query.single();

    let parent_id = commands.spawn().insert(Treats).id();
    commands.entity(world_id).push_children(&[parent_id]);

    map.walkable_positions.iter().for_each(|tile_position| {
        commands.spawn_bundle(CandyBundle {
            tile_position: tile_position.clone(),
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
            ..default()
        }).insert(Name::new("Candy"));
    })
}
