use bevy::{prelude::*, window::PrimaryWindow};

use super::{entity::player, physics, world};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            world::WorldPlugin,
            physics::GTransformPlugin,
            physics::RigidbodyPlugin,
            physics::ColliderPlugin,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup_game)
        .add_systems(Update, test_mouse);
    }
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn test_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = windows_query.single().cursor_position() {
            println!("oui ! [x: {}, y: {}]", position.x, position.y);
        } else {
            println!("oui ! mais je sais pas");
        }
    }
}
