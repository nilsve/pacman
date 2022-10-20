use bevy::prelude::*;
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

impl From<&EntityPosition> for TilePosition {
    fn from(entity_position: &EntityPosition) -> Self {
        Self {
            x: entity_position.x.round() as u32,
            y: entity_position.y.round() as u32,
        }
    }
}

impl From<EntityPosition> for TilePosition {
    fn from(entity_position: EntityPosition) -> Self {
        Self {
            x: entity_position.x.round() as u32,
            y: entity_position.y.round() as u32,
        }
    }
}

impl From<TilePosition> for EntityPosition {
    fn from(tile_position: TilePosition) -> Self {
        Self {
            x: tile_position.x as f32,
            y: tile_position.y as f32,
        }
    }
}

impl From<&TilePosition> for EntityPosition {
    fn from(tile_position: &TilePosition) -> Self {
        Self {
            x: tile_position.x as f32,
            y: tile_position.y as f32,
        }
    }
}
