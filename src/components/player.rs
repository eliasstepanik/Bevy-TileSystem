use bevy::input::ButtonInput;
use bevy::input::mouse::MouseWheel;
use bevy::log::info;
use bevy::prelude::{Camera, Camera2dBundle, Commands, Component, EventReader, GlobalTransform, KeyCode, Query, Res, Time, Transform, Vec3, With, Without};
use bevy::utils::default;
use crate::components::world::{CAMERA_SCALE, PLAYER_SPEED};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub position: Vec3,
}

impl Player {
    pub fn new(speed: f32, position: Vec3) -> Self {
        Player { speed, position }
    }
}

pub fn setup(mut commands: Commands) {
    // Setup camera
    info!("Adding Camera");
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::splat(CAMERA_SCALE), // Zoom in by reducing the scale (smaller scale means a larger view)
            ..default()
        },
        ..default()
    });


    // Setup player with initial position
    info!("Spawning player");
    commands.spawn((
        Player::new(PLAYER_SPEED, Vec3::new(0.0, 0.0, 0.0)), // Initial player speed and position
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GlobalTransform::default(),
    ));
}

pub fn update_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let delta_time = time.delta_seconds(); // Get the time elapsed between frames

    for (mut player, mut player_transform) in query.iter_mut() {
        // Update the player position based on input
        if keyboard_input.pressed(KeyCode::KeyW) {
            player.position.y += player.speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            player.position.y -= player.speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.position.x -= player.speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.position.x += player.speed * delta_time;
        }

        // Update the player's transform
        player_transform.translation = player.position;

        // Adjust speed exponentially with mouse wheel
        for event in mouse_wheel_events.read() {
            // Define a base factor for exponential change
            let base_factor = 1.1f32; // Adjust this value to control sensitivity

            // Calculate the factor to multiply with speed
            let factor = base_factor.powf(event.y);

            // Update the controller speed exponentially
            player.speed *= factor;

            // Ensure the speed doesn't go below a minimum value
            if player.speed < 0.01 {
                player.speed = 0.01;
            }
        }
    }



    // Update the camera transform to match the player's position
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok((player, _)) = query.get_single() {
            camera_transform.translation.x = player.position.x;
            camera_transform.translation.y = player.position.y;
        }
    }
}