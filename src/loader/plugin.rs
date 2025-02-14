use bevy::{
    app::{App, Plugin, PreStartup},
    asset::{AssetServer, Assets},
    math::UVec2,
    prelude::{Res, ResMut},
    sprite::TextureAtlasLayout,
};

use super::{
    asset_descriptions::{AssetDescription, AtlasDescription},
    assets::{AtlasAsset, TextureAsset, TextureAssets},
    datas,
};

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
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
    for asset in datas::TEXTURE_ASSET_DATAS {
        let texture = load_texture(&asset_server, asset);
        texture_assets.textures.insert(asset.id, texture);
    }

    for asset in datas::ATLAS_ASSET_DATAS {
        let atlas = load_atlas(&asset_server, &mut texture_atlases, asset);
        texture_assets.atlases.insert(asset.id, atlas);
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
