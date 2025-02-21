use crate::loader::texture::texture_id::TilesetId;

use super::block_type::BlockType;

#[derive(Clone, Copy, Default)]
pub struct Block {
    block_type: BlockType,
    is_collidable: bool,
}

impl Block {
    pub const fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            is_collidable: block_type.is_collidable(),
        }
    }

    pub const fn get_atlas_index(&self) -> usize {
        self.block_type.get_atlas_index()
    }

    pub const fn get_atlas_id(&self) -> Option<TilesetId> {
        self.block_type.get_atlas_id()
    }

    pub const fn get_type(&self) -> &BlockType {
        &self.block_type
    }

    pub const fn is_collidable(&self) -> &bool {
        &self.is_collidable
    }
}
