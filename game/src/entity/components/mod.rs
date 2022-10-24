use std::ops::Deref;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};

#[derive(Inspectable, Debug, Clone, Reflect, Default, Eq, PartialEq, Copy)]
pub enum EntityDirections {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CurrentDirection(pub Option<EntityDirections>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DirectionChange(pub Option<EntityDirections>);

impl Deref for CurrentDirection {
    type Target = Option<EntityDirections>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DirectionChange {
    type Target = Option<EntityDirections>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

