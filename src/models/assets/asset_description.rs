use crate::models::assets::texture::TextureId;

pub type AssetPath = &'static str;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetId {
    Texture(TextureId),
}

pub trait AssetDescription {
    fn get_id(&self) -> &AssetId;
    fn get_path(&self) -> AssetPath;
}
