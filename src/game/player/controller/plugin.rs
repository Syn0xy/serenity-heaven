use bevy::{
    app::{App, Plugin, Update},
    ecs::{schedule::IntoSystemConfigs, system::Query},
    input::ButtonInput,
    math::IVec2,
    prelude::{KeyCode, Res},
};

use crate::game::rigidbody::Rigidbody;

use super::PlayerController;

const SPEED: f32 = 10.;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_inputs, perform_movements).chain());
    }
}

fn handle_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut PlayerController>,
) {
    let controller = &mut controller_query.single_mut();
    let mut direction = IVec2::ZERO;

    for key in keys.get_pressed() {
        match key {
            KeyCode::KeyZ => direction.y += 1,
            KeyCode::KeyS => direction.y -= 1,
            KeyCode::KeyD => direction.x += 1,
            KeyCode::KeyQ => direction.x -= 1,
            _ => (),
        }
    }

    controller.direction = direction
}

fn perform_movements(mut controller_query: Query<(&mut Rigidbody, &PlayerController)>) {
    let (mut rigidbody, controller) = controller_query.single_mut();
    let direction = controller.direction;

    if direction == IVec2::ZERO {
        return;
    }

    let delta = direction.as_vec2().normalize_or_zero() * SPEED;

    rigidbody.apply_force(delta);
}
