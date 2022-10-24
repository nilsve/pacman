use bevy::prelude::*;

use crate::player::components::{Player, Score};
use crate::player::systems::{create_player, eat_candy, handle_input};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<Score>()
            .add_system(create_player)
            .add_system(handle_input)
            .add_system(eat_candy);
    }
}
