use std::{any::TypeId, collections::HashMap, ops::Deref};

use bevy::{
    asset::Handle, ecs::system::Resource, render::texture::Image, sprite::TextureAtlasLayout,
};

use crate::models::assets::{
    texture::{AtlasDescription, TextureDescription},
    AssetId,
};

#[derive(Resource, Debug, Default)]
pub struct TextureAssets {
    pub textures: HashMap<TypeId, TextureAsset<TextureDescription>>,
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
    pub fn get_texture<T: AssetId>(
        &self,
        asset_id: T,
    ) -> Option<&TextureAsset<TextureDescription>> {
        let type_id = TypeId::of::<T>();
        self.textures.get(&type_id)
    }

    pub fn get_atlas(&self, asset_id: impl Into<AssetId>) -> Option<&AtlasAsset> {
        self.atlases.get(&asset_id.into())
    }
}
