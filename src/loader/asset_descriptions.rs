use std::ops::Deref;

use super::datas::AssetName;

pub type AssetPath = &'static str;

#[derive(Debug, Clone, Copy)]
pub struct AssetDescription {
    pub name: AssetName,
    pub path: AssetPath,
}

#[derive(Debug, Clone, Copy)]
pub struct AtlasDescription {
    pub desc: AssetDescription,
    pub pixel_size: u32,
    pub columns: u32,
    pub rows: u32,
}

impl Deref for AtlasDescription {
    type Target = AssetDescription;
    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}
