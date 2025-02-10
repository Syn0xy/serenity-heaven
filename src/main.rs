use bevy::{prelude::*, window::PresentMode};
use serenity_heaven::{display, game, loader, resolution};

const WINDOW_TITLE: &str = "Serenity Heaven";

fn main() {
    let window = Window {
        title: WINDOW_TITLE.to_string(),
        position: WindowPosition::Centered(MonitorSelection::Primary),
        present_mode: PresentMode::Immediate,
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            resolution::ResolutionPlugin,
            loader::LoaderPlugin,
            game::GamePlugin,
            display::DisplayPlugin,
        ))
        .run();
}
