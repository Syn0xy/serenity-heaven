use bevy::prelude::*;

use crate::systems::game::player;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player::handle_inputs, player::perform_movements).chain(),
        );
    }
}
