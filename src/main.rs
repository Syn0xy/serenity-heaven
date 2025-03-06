pub mod constants;
pub mod models;
pub mod plugins;

use bevy::{prelude::*, window::PresentMode};
use plugins::{assets, display, game};

const WINDOW_TITLE: &str = "Serenity Heaven";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            assets::LoaderPlugin,
            game::GamePlugin,
            display::DisplayPlugin,
        ))
        .run();
}
