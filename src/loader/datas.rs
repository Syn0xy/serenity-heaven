use super::asset_descriptions::{AtlasDescription, TextureDescription};

pub const TILE_SIZE: u32 = 16;

pub mod asset_name {
    pub mod tileset {
        pub const NATURE: usize = 0;
    }
    pub mod player {
        pub const IDLE: usize = 1;
    }
    pub mod slime {
        pub const IDLE: usize = 2;
        pub const DEAD: usize = 3;
        pub const JUMP: usize = 4;
    }
}

pub(super) const TEXTURE_ASSET_DATAS: &[TextureDescription] = &[
    // PLAYER
    TextureDescription {
        id: asset_name::player::IDLE,
        path: "entities/player/player_idle.png",
    },
    // SLIME
    TextureDescription {
        id: asset_name::slime::IDLE,
        path: "entities/slime/slime_idle.png",
    },
    TextureDescription {
        id: asset_name::slime::DEAD,
        path: "entities/slime/slime_dead.png",
    },
    TextureDescription {
        id: asset_name::slime::JUMP,
        path: "entities/slime/slime_jump.png",
    },
];

pub(super) const ATLAS_ASSET_DATAS: &[AtlasDescription] = &[AtlasDescription {
    desc: TextureDescription {
        id: asset_name::tileset::NATURE,
        path: "nature/tileset.png",
    },
    pixel_size: TILE_SIZE,
    columns: 25,
    rows: 4,
}];
