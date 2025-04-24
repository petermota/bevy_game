use bevy::prelude::*;

// Import systems and the functios
mod systems;
use systems::spawn_camera;

// 2D Camera related stuff
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

