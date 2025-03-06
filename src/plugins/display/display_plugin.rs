use bevy::prelude::*;

use super::{debug, ResolutionPlugin};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ResolutionPlugin, debug::DebugPlugin));
    }
}
