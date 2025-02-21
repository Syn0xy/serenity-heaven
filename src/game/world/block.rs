use std::ops::Deref;

use super::block_type::BlockType;

#[derive(Clone, Copy, Default)]
pub struct Block {
    block_type: BlockType,
}

impl Block {
    pub const fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }

    pub const fn get_type(&self) -> &BlockType {
        &self.block_type
    }
}

impl Deref for Block {
    type Target = BlockType;

    fn deref(&self) -> &Self::Target {
        &self.block_type
    }
}
