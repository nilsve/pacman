use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::render::texture::{ImageSampler, ImageSettings};
use bevy_inspector_egui::WorldInspectorPlugin;
use map_parser::GameMap;
use crate::camera::plugin::CameraPlugin;
use crate::entity::plugin::EntityPlugin;
use crate::graphics::plugin::GraphicsPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::world::Map;
use crate::world::plugin::WorldPlugin;

mod world;
mod camera;
mod graphics;
mod player;
mod entity;

fn main() {
    let map = include_str!("./map1.json");

    let mut app = App::new();
    app.insert_resource(ImageSettings {
        default_sampler: ImageSampler::nearest_descriptor(),
    }).add_plugins(DefaultPlugins);

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    app
        .insert_resource(Map::new(map.to_owned()))
        .add_plugin(WorldPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EntityPlugin)

        .run();
}
