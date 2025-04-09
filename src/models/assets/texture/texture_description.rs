use std::ops::Deref;

use crate::models::assets::{AssetDescription, AssetId, AssetPath};

#[derive(Debug, Clone)]
pub struct TextureDescription<ID> {
    pub id: ID,
    pub path: AssetPath,
}

#[derive(Debug, Clone)]
pub struct AtlasDescription<ID> {
    pub desc: TextureDescription<ID>,
    pub pixel_size: u32,
    pub columns: u32,
    pub rows: u32,
}

impl<ID> Deref for AtlasDescription<ID> {
    type Target = TextureDescription<ID>;
    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl<ID> AssetDescription for TextureDescription<ID> {
    fn get_id(&self) -> &ID {
        &self.id
    }

    fn get_path(&self) -> AssetPath {
        self.path
    }
}

impl<ID> AssetDescription for AtlasDescription<ID> {
    fn get_id(&self) -> &ID {
        &self.id
    }

    fn get_path(&self) -> AssetPath {
        self.path
    }
}
