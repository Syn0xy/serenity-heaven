use crate::models::assets::AssetId;

macro_rules! impl_into_asset_id {
    ($source:ty => $variant:path) => {
        impl Into<AssetId> for $source {
            fn into(self) -> AssetId {
                $variant(self).into()
            }
        }
    };
}

impl_into_asset_id!(TextureId => AssetId::Texture);
impl_into_asset_id!(StaticId => TextureId::Static);
impl_into_asset_id!(TilesetId => TextureId::Tileset);
impl_into_asset_id!(PlayerId => StaticId::Player);
impl_into_asset_id!(SlimeId => StaticId::Slime);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureId {
    Static(StaticId),
    Tileset(TilesetId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticId {
    Player(PlayerId),
    Slime(SlimeId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TilesetId {
    Grass,
    Detail,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerId {
    Test,
    Idle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SlimeId {
    Idle,
    Dead,
    Jump,
}
