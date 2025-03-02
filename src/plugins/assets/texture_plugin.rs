use bevy::app::{App, Plugin, PreStartup};

use crate::{resources::TextureAssets, systems::assets};

pub struct TextureLoaderPlugin;

impl Plugin for TextureLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TextureAssets::default())
            .add_systems(PreStartup, assets::load_textures);
    }
}
