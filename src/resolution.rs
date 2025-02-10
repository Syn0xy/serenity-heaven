use bevy::{
    app::{Plugin, PreStartup},
    math::Vec2,
    prelude::{Commands, Query, Resource},
    window::Window,
};

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    pub screen_dimensions: Vec2,
    pub pixel_ratio: f32,
}

fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();

    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),
        pixel_ratio: 2.0,
    });
}
