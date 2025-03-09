use bevy::prelude::*;

use crate::models::game::{
    physics::{GTransform, Rigidbody},
    player::Player,
};

pub struct DebugPlayerPlugin;

impl Plugin for DebugPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_display)
            .add_systems(Update, update_player_display);
    }
}

#[derive(Component)]
struct DebugText;

#[derive(Component)]
enum DebugLabel {
    Position,
    Velocity,
    Speed,
}

fn setup_player_display(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column, // Empile les textes verticalement
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                DebugText,
                DebugLabel::Position,
                TextBundle::from_section("Position: ?", text_style.clone()),
            ));

            parent.spawn((
                DebugText,
                DebugLabel::Velocity,
                TextBundle::from_section("Velocity: ?", text_style.clone()),
            ));

            parent.spawn((
                DebugText,
                DebugLabel::Speed,
                TextBundle::from_section("Speed: ?", text_style.clone()),
            ));
        });
}

fn update_player_display(
    player_query: Query<(&GTransform, &Rigidbody), With<Player>>,
    mut text_query: Query<(&DebugLabel, &mut Text), With<DebugText>>,
) {
    if let Ok((gtransform, rigidbody)) = player_query.get_single() {
        for (label, mut text) in &mut text_query {
            text.sections[0].value = match label {
                DebugLabel::Position => {
                    format!("Position: {:.2?}", gtransform.position)
                }
                DebugLabel::Velocity => {
                    format!("Velocity: {:.2?}", rigidbody.velocity)
                }
                DebugLabel::Speed => {
                    format!("Speed: {:.2?}", rigidbody.current_speed)
                }
            }
        }
    }
}
