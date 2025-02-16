use bevy::app::{App, Plugin};

use super::texture;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(texture::TextureLoaderPlugin);
    }
}
