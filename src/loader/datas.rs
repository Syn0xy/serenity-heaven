use super::AtlasDescription;

pub const TILE_SIZE: u32 = 16;

pub(super) const TEXTURE_ASSET_DATAS: &[(&'static str, &'static str)] =
    &[("player_idle", "entities/player/player_idle.png")];

pub(super) const ATLAS_ASSET_DATAS: &[(&'static str, &AtlasDescription)] = &[(
    "nature",
    &AtlasDescription {
        file_path: "nature/tileset.png",
        tile_size: TILE_SIZE,
        columns: 25,
        rows: 4,
    },
)];
