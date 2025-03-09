use bevy::prelude::*;

use super::{diagnostic, information, physics};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            diagnostic::DebugFpsPlugin,
            information::DebugPlayerPlugin,
            physics::DebugColliderPlugin,
        ));
    }
}
