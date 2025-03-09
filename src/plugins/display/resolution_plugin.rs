use bevy::prelude::*;

use crate::models::display::Resolution;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();

    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),
        pixel_ratio: 4.0,
    });
}
