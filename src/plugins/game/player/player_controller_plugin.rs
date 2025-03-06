use bevy::prelude::*;

use crate::{
    constants::player_datas,
    models::game::{
        physics::{ForceMode, Rigidbody},
        player::PlayerController,
    },
};

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
            KeyCode::KeyW => direction.y += 1,
            KeyCode::KeyS => direction.y -= 1,
            KeyCode::KeyD => direction.x += 1,
            KeyCode::KeyA => direction.x -= 1,
            _ => (),
        }
    }

    controller.direction = direction
}

fn perform_movements(
    mut controller_query: Query<(&mut Rigidbody, &PlayerController)>,
    time: Res<Time>,
) {
    let (mut rigidbody, controller) = controller_query.single_mut();
    let direction = controller.direction;
    let direction_normalize = direction.as_vec2().normalize_or_zero();
    let delta_time = time.delta_seconds();

    let target_velocity = direction_normalize * player_datas::PLAYER_MAX_SPEED;
    let max_velocity = player_datas::PLAYER_MAX_ACCEL * delta_time;
    let new_velocity = rigidbody.velocity.lerp(target_velocity, max_velocity);

    rigidbody.add_force(new_velocity, ForceMode::Acceleration);
}
