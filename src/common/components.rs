use bevy::prelude::*;

/// Component representing the velocity of an entity.
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Component representing the movement speed of an entity.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.0) // Default move speed
    }
}
