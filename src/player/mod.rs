use bevy::prelude::*;

mod components;
mod systems;

use systems::{apply_velocity, handle_player_input, spawn_player};

/// Plugin responsible for player-related logic.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add systems
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    handle_player_input,
                    apply_velocity.after(handle_player_input), // Ensure input is handled before velocity is applied
                ),
            );
    }
}
