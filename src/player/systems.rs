use crate::{
    common::components::{Speed, Velocity}, // Import generic components
    player::components::Player,            // Import player marker
};
use bevy::prelude::*;
use bevy::time::Time; // Explicitly import Time

/// Spawns the player entity with necessary components during startup.
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.7, 0.1), Vec2::new(50.0, 50.0)),
        Player,
        Velocity::default(), // Use generic Velocity
        Speed::default(),    // Use generic Speed
        Name::new("Player"), // Add a name for easier debugging
    ));
}

/// Handles keyboard input (WASD) to update the player's velocity.
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Use ButtonInput instead of Input
    mut query: Query<&mut Velocity, With<Player>>, // Query for generic Velocity
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = 0.0;
        velocity.y = 0.0;

        if keyboard_input.pressed(KeyCode::KeyW) {
            // Use KeyW
            velocity.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            // Use KeyS
            velocity.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            // Use KeyA
            velocity.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            // Use KeyD
            velocity.x += 1.0;
        }

        // Normalize diagonal movement (optional, but good practice)
        if velocity.x != 0.0 && velocity.y != 0.0 {
            let factor = 1.0 / 2.0f32.sqrt();
            velocity.x *= factor;
            velocity.y *= factor;
        }
    }
}

/// Applies the velocity to the player's transform to move them.
pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity, &Speed), With<Player>>, // Query for generic Velocity and Speed
    time: Res<Time>,
) {
    if let Ok((mut transform, velocity, move_speed)) = query.get_single_mut() {
        transform.translation.x += velocity.x * move_speed.0 * time.delta_secs();
        transform.translation.y += velocity.y * move_speed.0 * time.delta_secs();
    }
}
