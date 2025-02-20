use crate::loader::asset_id::AssetId;

use super::texture_id::*;

use super::description::{AtlasDescription, TextureDescription};

pub const TILE_SIZE: u32 = 16;

pub(super) const TEXTURE_ASSET_DATAS: &[TextureDescription] = &[
    // PLAYER
    TextureDescription {
        id: AssetId::Texture(TextureId::Static(StaticId::Player(PlayerId::Idle))),
        path: "entities/player/player_idle.png",
    },
    TextureDescription {
        id: AssetId::Texture(TextureId::Static(StaticId::Player(PlayerId::Test))),
        path: "entities/player/player_test.png",
    },
    // SLIME
    TextureDescription {
        id: AssetId::Texture(TextureId::Static(StaticId::Slime(SlimeId::Idle))),
        path: "entities/slime/slime_idle.png",
    },
    TextureDescription {
        id: AssetId::Texture(TextureId::Static(StaticId::Slime(SlimeId::Dead))),
        path: "entities/slime/slime_dead.png",
    },
    TextureDescription {
        id: AssetId::Texture(TextureId::Static(StaticId::Slime(SlimeId::Jump))),
        path: "entities/slime/slime_jump.png",
    },
];

pub(super) const ATLAS_ASSET_DATAS: &[AtlasDescription] = &[
    AtlasDescription {
        desc: TextureDescription {
            id: AssetId::Texture(TextureId::Tileset(TilesetId::Grass)),
            path: "tileset/grass.png",
        },
        pixel_size: TILE_SIZE,
        columns: 4,
        rows: 4,
    },
    AtlasDescription {
        desc: TextureDescription {
            id: AssetId::Texture(TextureId::Tileset(TilesetId::Detail)),
            path: "tileset/detail.png",
        },
        pixel_size: TILE_SIZE,
        columns: 25,
        rows: 4,
    },
];
