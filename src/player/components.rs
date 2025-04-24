use bevy::prelude::*;

/// Marker component for the player entity.
#[derive(Component)]
pub struct Player;

/// Component representing the velocity of an entity.
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerVelocity {
    pub x: f32,
    pub y: f32,
}

/// Component representing the movement speed of an entity.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerSpeed(pub f32);

impl Default for PlayerSpeed {
    fn default() -> Self {
        Self(500.0) // Default move speed
    }
}