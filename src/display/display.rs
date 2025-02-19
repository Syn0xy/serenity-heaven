use bevy::prelude::*;

use super::{debug, resolution};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((resolution::ResolutionPlugin, debug::DisplayDebugPlugin));
    }
}
