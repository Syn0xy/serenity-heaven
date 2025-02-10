use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        query::With,
        schedule::IntoSystemConfigs,
        system::{Query, ResMut, Resource},
    },
    input::ButtonInput,
    math::IVec2,
    prelude::{KeyCode, Res},
    time::Time,
};

use crate::game::GTransform;

use super::Player;

const SPEED: f32 = 10.;

#[derive(Default, Resource)]
struct PlayerController {
    direction: IVec2,
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerController::default())
            .add_systems(Update, (handle_inputs, perform_movements).chain());
    }
}

fn handle_inputs(keys: Res<ButtonInput<KeyCode>>, mut controller: ResMut<PlayerController>) {
    let direction = &mut controller.direction;

    direction.x = 0;
    direction.y = 0;

    for key in keys.get_pressed() {
        match key {
            KeyCode::KeyZ => direction.y += 1,
            KeyCode::KeyS => direction.y -= 1,
            KeyCode::KeyD => direction.x += 1,
            KeyCode::KeyQ => direction.x -= 1,
            _ => (),
        }
    }
}

fn perform_movements(
    controller: Res<PlayerController>,
    mut player_query: Query<&mut GTransform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = player_query.single_mut();
    let direction = controller.direction;

    if direction == IVec2::ZERO {
        return;
    }

    let delta_speed = time.delta_seconds() * SPEED;
    let delta = direction.as_vec2().normalize_or_zero() * delta_speed;

    transform.position += delta;
}
