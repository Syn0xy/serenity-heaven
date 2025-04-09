use std::hash::Hash;

pub type AssetPath = &'static str;

pub trait AssetId: Hash {}

pub trait AssetDescription {
    fn get_id(&self) -> impl AssetId;
    fn get_path(&self) -> AssetPath;
}
