use bevy::app::{Plugin, PreStartup};

use crate::systems::display;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, display::setup_resolution);
    }
}
