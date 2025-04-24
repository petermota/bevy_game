use bevy::prelude::*;

mod components;
mod systems;

use components::{PlayerVelocity, PlayerSpeed};
use systems::{spawn_player, handle_player_input, apply_velocity};

/// Plugin responsible for player-related logic.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components for reflection (useful for editor integration, saving/loading)
            .register_type::<PlayerVelocity>()
            .register_type::<PlayerSpeed>()
            // Add systems
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                handle_player_input,
                apply_velocity.after(handle_player_input) // Ensure input is handled before velocity is applied
            ));
    }
}