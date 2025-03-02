use bevy::{
    ecs::system::{Commands, Query},
    math::Vec2,
    window::Window,
};

use crate::resources::Resolution;

pub fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();

    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),
        pixel_ratio: 2.0,
    });
}
