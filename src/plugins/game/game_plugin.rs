use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    plugins::game::{physics, player, world},
    systems::game,
};

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
        .add_systems(Startup, game::setup_game)
        .add_systems(Update, test_mouse);
    }
}

fn test_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            println!("oui ! [x: {}, y: {}]", position.x, position.y);
        } else {
            println!("oui ! mais je sais pas");
        }
    }
}
