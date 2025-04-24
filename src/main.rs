use bevy::prelude::*;

mod player; // Declare the player module
use player::PlayerPlugin; // Import the plugin


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin // Keep existing player plugin

        ))
        .run();
}
