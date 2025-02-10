use std::collections::HashMap;

use bevy::{
    app::{App, Plugin, PreStartup},
    asset::{AssetServer, Assets, Handle},
    math::UVec2,
    prelude::{Image, Res, ResMut, Resource},
    sprite::TextureAtlasLayout,
};

use super::datas;

#[derive(Debug, Default, Resource)]
pub struct TextureAssets {
    pub textures: HashMap<&'static str, Handle<Image>>,
    pub atlases: HashMap<&'static str, AtlasAsset>,
}

#[derive(Debug)]
pub struct AtlasAsset {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Debug, Default)]
pub struct AtlasDescription {
    pub file_path: &'static str,
    pub tile_size: u32,
    pub columns: u32,
    pub rows: u32,
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TextureAssets::default())
            .add_systems(PreStartup, load_textures);
    }
}

impl TextureAssets {
    pub fn get_texture(&self, texture_name: &str) -> Option<&Handle<Image>> {
        self.textures.get(texture_name)
    }

    pub fn get_atlas(&self, atlas_name: &str) -> Option<&AtlasAsset> {
        self.atlases.get(atlas_name)
    }
}

fn load_textures(
    mut texture_assets: ResMut<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for &(texture_name, texture_path) in datas::TEXTURE_ASSET_DATAS {
        let texture_handle = asset_server.load(texture_path);
        texture_assets.textures.insert(texture_name, texture_handle);
    }

    for &(atlas_name, description) in datas::ATLAS_ASSET_DATAS {
        let atlas = load_atlas(&asset_server, &mut texture_atlases, description);
        texture_assets.atlases.insert(atlas_name, atlas);
    }
}

fn load_atlas(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    atlas_description: &AtlasDescription,
) -> AtlasAsset {
    let texture_handle = asset_server.load(atlas_description.file_path);
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(atlas_description.tile_size),
        atlas_description.columns,
        atlas_description.rows,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    AtlasAsset {
        image: texture_handle,
        layout: texture_atlas_handle,
    }
}
