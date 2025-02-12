use super::asset_descriptions::{AssetDescription, AtlasDescription};

pub const TILE_SIZE: u32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetName {
    Player(PlayerTexture),
    Slime(SlimeTexture),
    Tileset(NatureTexture),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerTexture {
    Idle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlimeTexture {
    Idle,
    Dead,
    Jump,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NatureTexture {
    Tileset,
}

pub(super) const TEXTURE_ASSET_DATAS: &[AssetDescription] = &[
    // PLAYER
    AssetDescription {
        name: AssetName::Player(PlayerTexture::Idle),
        path: "entities/player/player_idle.png",
    },
    // SLIME
    AssetDescription {
        name: AssetName::Slime(SlimeTexture::Idle),
        path: "entities/slime/slime_idle.png",
    },
    AssetDescription {
        name: AssetName::Slime(SlimeTexture::Dead),
        path: "entities/slime/slime_dead.png",
    },
    AssetDescription {
        name: AssetName::Slime(SlimeTexture::Jump),
        path: "entities/slime/slime_jump.png",
    },
];

pub(super) const ATLAS_ASSET_DATAS: &[AtlasDescription] = &[AtlasDescription {
    desc: AssetDescription {
        name: AssetName::Tileset(NatureTexture::Tileset),
        path: "nature/tileset.png",
    },
    pixel_size: TILE_SIZE,
    columns: 25,
    rows: 4,
}];
