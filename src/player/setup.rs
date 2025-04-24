use bevy::prelude::*;
use crate::player::components::{Player, PlayerVelocity, PlayerSpeed};

/// Spawns the player entity with necessary components during startup.
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.7, 0.1), Vec2::new(50.0, 50.0)),
        Player,
        PlayerVelocity::default(),
        PlayerSpeed::default(),
        Name::new("Player"), // Add a name for easier debugging
    ));
}