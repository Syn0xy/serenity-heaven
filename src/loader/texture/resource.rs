use std::{collections::HashMap, ops::Deref};

use bevy::{
    asset::Handle, ecs::system::Resource, render::texture::Image, sprite::TextureAtlasLayout,
};

use crate::loader::asset_id::AssetId;

use super::description::{AtlasDescription, TextureDescription};

#[derive(Resource, Debug, Default)]
pub struct TextureAssets {
    pub textures: HashMap<AssetId, TextureAsset<TextureDescription>>,
    pub atlases: HashMap<AssetId, AtlasAsset>,
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
    pub fn get_texture<T: Into<AssetId>>(
        &self,
        asset_id: T,
    ) -> Option<&TextureAsset<TextureDescription>> {
        self.textures.get(&asset_id.into())
    }

    pub fn get_atlas<T: Into<AssetId>>(&self, asset_id: T) -> Option<&AtlasAsset> {
        self.atlases.get(&asset_id.into())
    }
}
