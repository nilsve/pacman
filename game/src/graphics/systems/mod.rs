use bevy::prelude::*;
use crate::graphics::data::PacmanSheet;
use crate::world::components::EntityPosition;

const TILE_SIZE: f32 = 16.0;

pub fn load_textures(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    println!("load textures");
    let image = assets.load("tilesheet.png");
    let atlas = TextureAtlas::from_grid_with_padding(image, Vec2::new(16.0, 16.0), 9, 9, Vec2::new(1.0, 1.0), Vec2::new(0.0, 0.0));

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(PacmanSheet(atlas_handle));
}

pub fn sync_positions(
    mut query: Query<(&mut Transform, &EntityPosition), Changed<EntityPosition>>
) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.x * TILE_SIZE;
        transform.translation.y = position.y * TILE_SIZE;
    }
}
