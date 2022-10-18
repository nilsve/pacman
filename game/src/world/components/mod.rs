use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};
use crate::entity::components::EntityDirections;

#[derive(Component, Reflect, Default, Copy, Clone, Debug)]
#[reflect(Component)]
pub struct EntityPosition {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Component, Reflect, Default, Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[reflect(Component)]
pub struct TilePosition {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl TilePosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
    pub fn apply_direction(&self, direction: &EntityDirections) -> Self {
        match direction {
            EntityDirections::Up => Self::new(self.x, self.y + 1),
            EntityDirections::Down => Self::new(self.x, self.y - 1),
            EntityDirections::Left => Self::new(self.x - 1, self.y),
            EntityDirections::Right => Self::new(self.x + 1, self.y),
        }
    }

    pub fn into_entity_position(self) -> EntityPosition {
        self.into()
    }
}

impl EntityPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
    pub fn apply_direction(&self, direction: &EntityDirections) -> Self {
        match direction {
            EntityDirections::Up => Self::new(self.x, self.y + 1.0),
            EntityDirections::Down => Self::new(self.x, self.y - 1.0),
            EntityDirections::Left => Self::new(self.x - 1.0, self.y),
            EntityDirections::Right => Self::new(self.x + 1.0, self.y),
        }
    }

    pub fn into_tile_position(self) -> TilePosition {
        self.into()
    }
}

impl Into<TilePosition> for EntityPosition {
    fn into(self) -> TilePosition {
        TilePosition {
            x: self.x.round() as u32,
            y: self.y.round() as u32,
        }
    }
}

impl Into<EntityPosition> for TilePosition {
    fn into(self) -> EntityPosition {
        EntityPosition {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl Into<TilePosition> for &EntityPosition {
    fn into(self) -> TilePosition {
        TilePosition {
            x: self.x.round() as u32,
            y: self.y.round() as u32,
        }
    }
}

impl Into<EntityPosition> for &TilePosition {
    fn into(self) -> EntityPosition {
        EntityPosition {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

#[derive(Component, Default)]
pub struct Tiles;

#[derive(Bundle)]
pub struct Tile {
    pub(crate) position: TilePosition,

    pub(crate) tile_type: TileType,

    pub name: Name,
}
#[derive(Inspectable, Debug, Clone, Reflect, Default, Eq, PartialEq, Copy)]
pub enum TileTypes {
    #[default]
    Wall,
    PlayerSpawn,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TileType(pub TileTypes);

