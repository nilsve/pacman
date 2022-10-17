use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};

#[derive(Inspectable, Debug, Clone, Reflect, Default)]
pub enum EntityDirections {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EntityDirection(pub EntityDirections);

#[derive(Component, Default)]
pub struct Entity;
