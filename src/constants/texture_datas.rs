use crate::models::assets::texture::*;

pub const TILE_SIZE: u32 = 16;
pub const TILE_SIZE_F32: f32 = TILE_SIZE as f32;

pub const TEXTURE_ASSET_DATAS: &[TextureDescription] = &[
    // PLAYER
    TextureDescription {
        id: PlayerId::Idle,
        path: "entities/player/player_idle.png",
    },
    TextureDescription {
        id: PlayerId::Test,
        path: "entities/player/player_test.png",
    },
    // SLIME
    TextureDescription {
        id: SlimeId::Idle,
        path: "entities/slime/slime_idle.png",
    },
    TextureDescription {
        id: SlimeId::Dead,
        path: "entities/slime/slime_dead.png",
    },
    TextureDescription {
        id: SlimeId::Jump,
        path: "entities/slime/slime_jump.png",
    },
];

pub const ATLAS_ASSET_DATAS: &[AtlasDescription] = &[
    AtlasDescription {
        desc: TextureDescription {
            id: TilesetId::Grass,
            path: "tileset/grass.png",
        },
        pixel_size: TILE_SIZE,
        columns: 4,
        rows: 4,
    },
    AtlasDescription {
        desc: TextureDescription {
            id: TilesetId::Detail,
            path: "tileset/detail.png",
        },
        pixel_size: TILE_SIZE,
        columns: 25,
        rows: 4,
    },
];
