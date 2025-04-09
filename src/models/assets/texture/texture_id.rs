use crate::models::assets::AssetId;

macro_rules! impl_into_asset_id {
    ($source:ty) => {
        impl AssetId for $source {}
    };
}

impl_into_asset_id!(TilesetId);
impl_into_asset_id!(PlayerId);
impl_into_asset_id!(SlimeId);

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
