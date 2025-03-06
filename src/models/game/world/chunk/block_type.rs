use BlockType::*;

use crate::models::assets::texture::TilesetId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    #[default]
    Air,
    Grass,
    Sand,
    Water,
    DeadTree,
    Branch,
    Flower,
    Bush,
    Mushroom,
    Plant,
    Rock,
    Pebble,
}

impl BlockType {
    pub const fn is_collidable(&self) -> bool {
        match self {
            Water | DeadTree | Rock | Bush => true,
            _ => false,
        }
    }

    pub const fn is_air(&self) -> bool {
        matches!(self, &Air)
    }

    pub const fn get_atlas_index(&self) -> usize {
        match self {
            Sand => 1,
            Water => 2,
            DeadTree => 61,
            Branch => 59,
            Flower => 53,
            Bush => 38,
            Mushroom => 88,
            Plant => 72,
            Rock => 77,
            Pebble => 82,
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
