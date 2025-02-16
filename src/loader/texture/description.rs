use std::ops::Deref;

use crate::loader::{
    asset_description::{AssetDescription, AssetPath},
    asset_id::AssetId,
};

#[derive(Debug, Clone)]
pub struct TextureDescription {
    pub id: AssetId,
    pub path: AssetPath,
}

#[derive(Debug, Clone)]
pub struct AtlasDescription {
    pub desc: TextureDescription,
    pub pixel_size: u32,
    pub columns: u32,
    pub rows: u32,
}

impl Deref for AtlasDescription {
    type Target = TextureDescription;
    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl AssetDescription for TextureDescription {
    fn get_id(&self) -> &AssetId {
        &self.id
    }

    fn get_path(&self) -> AssetPath {
        self.path
    }
}

impl AssetDescription for AtlasDescription {
    fn get_id(&self) -> &AssetId {
        &self.id
    }

    fn get_path(&self) -> AssetPath {
        self.path
    }
}
