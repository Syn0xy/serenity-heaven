use super::texture::texture_id::TextureId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetId {
    Texture(TextureId),
}
