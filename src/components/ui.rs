use bevy::asset::AssetServer;
use bevy::prelude::{default, Camera, Color, Commands, Component, Query, Res, Text, TextBundle, TextStyle, Time, Transform, Val, With};
use bevy::ui::{PositionType, Style};
use crate::components::player::Player;

#[derive(Component)]
pub struct SpeedDisplay;

#[derive(Component)]
pub struct FpsText;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Spawn UI text for debugging speed
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Speed: 5.0\nFPS: 60", // Initial text
                TextStyle {
                    font: asset_server.load("fonts/minecraft_font.ttf"), // Load a font
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        SpeedDisplay,
    ));
}

pub fn update(
    player_query: Query<&Player>,
    mut query_text: Query<&mut Text, With<SpeedDisplay>>,
    time: Res<Time>,
) {
    let player = player_query.single();
    let mut text = query_text.single_mut();

    let fps = 1.0 / time.delta_seconds();

    // Update the text with the current speed and position
    text.sections[0].value = format!(
        "Speed: {:.2}\nPosition: ({:.2}, {:.2}, {:.2})\nFps:{:2}",
        player.speed,
        player.position.x,
        player.position.y,
        player.position.z,
        fps
    );

}
