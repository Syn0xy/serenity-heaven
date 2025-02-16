use bevy::{
    app::{Plugin, PreStartup},
    asset::{AssetServer, Assets},
    ecs::system::{Res, ResMut},
    math::UVec2,
    sprite::TextureAtlasLayout,
};

use crate::loader::asset_description::AssetDescription;

use super::{
    data,
    description::AtlasDescription,
    resource::{AtlasAsset, TextureAsset, TextureAssets},
};

pub struct TextureLoaderPlugin;

impl Plugin for TextureLoaderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(TextureAssets::default())
            .add_systems(PreStartup, load_textures);
    }
}

fn load_texture<T>(asset_server: &Res<AssetServer>, asset_description: &T) -> TextureAsset<T>
where
    T: AssetDescription + Clone,
{
    TextureAsset {
        desc: asset_description.clone(),
        image: asset_server.load(asset_description.get_path()),
    }
}

fn load_textures(
    mut texture_assets: ResMut<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for asset in data::TEXTURE_ASSET_DATAS {
        let texture = load_texture(&asset_server, asset);
        texture_assets.textures.insert(asset.id.clone(), texture);
    }

    for asset in data::ATLAS_ASSET_DATAS {
        let atlas = load_atlas(&asset_server, &mut texture_atlases, asset);
        texture_assets.atlases.insert(asset.id.clone(), atlas);
    }
}

fn load_atlas(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    atlas_description: &AtlasDescription,
) -> AtlasAsset {
    let texture = load_texture(&asset_server, atlas_description);
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(atlas_description.pixel_size),
        atlas_description.columns,
        atlas_description.rows,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    AtlasAsset {
        asset: texture,
        layout: texture_atlas_handle,
    }
}
