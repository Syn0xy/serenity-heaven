use std::ops::Deref;

pub type AssetPath = &'static str;

pub trait AssetDescription {
    fn get_id(&self) -> usize;
    fn get_path(&self) -> AssetPath;
}

#[derive(Debug, Clone, Copy)]
pub struct TextureDescription {
    pub id: usize,
    pub path: AssetPath,
}

#[derive(Debug, Clone, Copy)]
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
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_path(&self) -> AssetPath {
        self.path
    }
}

impl AssetDescription for AtlasDescription {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_path(&self) -> AssetPath {
        self.path
    }
}
