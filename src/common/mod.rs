use bevy::prelude::*;

// Declare submodules within the common module
pub mod components;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components for reflection (useful for editor integration, saving/loading)
            // Register the generic components used by the player
            .register_type::<components::Velocity>()
            .register_type::<components::Speed>();
    }
}
