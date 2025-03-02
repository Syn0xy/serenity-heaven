use bevy::prelude::*;

use crate::{plugins::game::player::PlayerControllerPlugin, systems::game::player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerControllerPlugin)
            .add_systems(Startup, player::setup_player);
    }
}
