use super::asset_id::AssetId;

pub type AssetPath = &'static str;

pub trait AssetDescription {
    fn get_id(&self) -> &AssetId;
    fn get_path(&self) -> AssetPath;
}
