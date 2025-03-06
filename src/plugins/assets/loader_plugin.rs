use bevy::app::{App, Plugin};

use super::TextureLoaderPlugin;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextureLoaderPlugin);
    }
}
