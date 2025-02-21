use crate::loader::texture::texture_id::TilesetId;

use BlockType::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    #[default]
    Grass,
    Sand,
    Water,
}

impl BlockType {
    pub const fn is_collidable(&self) -> bool {
        match self {
            Water => true,
            _ => false,
        }
    }

    pub const fn get_atlas_index(&self) -> usize {
        match self {
            Sand => 1,
            Water => 2,
            _ => 0,
        }
    }

    pub const fn get_atlas_id(&self) -> Option<TilesetId> {
        match self {
            Grass => Some(TilesetId::Grass),
            _ => None,
        }
    }
}
