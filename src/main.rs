use bevy::prelude::*;

mod camera;
mod common;
mod player;

use camera::CameraPlugin;
use common::CommonPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CommonPlugin,
            CameraPlugin,
            PlayerPlugin,
        ))
        .run();
}
