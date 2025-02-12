use std::{collections::HashMap, ops::Deref};

use bevy::{
    asset::Handle, ecs::system::Resource, render::texture::Image, sprite::TextureAtlasLayout,
};

use super::{
    asset_descriptions::{AssetDescription, AtlasDescription},
    datas::AssetName,
};

#[derive(Debug, Default, Resource)]
pub struct TextureAssets {
    pub textures: HashMap<AssetName, TextureAsset<AssetDescription>>,
    pub atlases: HashMap<AssetName, AtlasAsset>,
}

#[derive(Debug, Clone)]
pub struct TextureAsset<T> {
    pub desc: T,
    pub image: Handle<Image>,
}

#[derive(Debug, Clone)]
pub struct AtlasAsset {
    pub asset: TextureAsset<AtlasDescription>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl<T> Deref for TextureAsset<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl Deref for AtlasAsset {
    type Target = TextureAsset<AtlasDescription>;
    fn deref(&self) -> &Self::Target {
        &self.asset
    }
}

impl TextureAssets {
    pub fn get_texture(&self, texture_name: &AssetName) -> Option<&TextureAsset<AssetDescription>> {
        self.textures.get(texture_name)
    }

    pub fn get_atlas(&self, atlas_name: &AssetName) -> Option<&AtlasAsset> {
        self.atlases.get(atlas_name)
    }
}
