use bevy::prelude::*;

use crate::plugins::debug::DisplayDebugPlugin;

use super::ResolutionPlugin;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ResolutionPlugin, DisplayDebugPlugin));
    }
}
