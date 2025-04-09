use bevy::prelude::*;

use crate::constants::chunk_datas::*;

use super::{Block, BlockType, ChunkData};

#[derive(Component)]
pub struct Chunk {
    pub _data: ChunkData,
    pub blocks: [[Block; CHUNK_LENGTH]; CHUNK_HEIGHT],
}

impl Chunk {
    pub fn new(_data: ChunkData) -> Self {
        let blocks = Chunk::generate_blocks(&_data);
        Self { _data, blocks }
    }

    pub fn generate_blocks(data: &ChunkData) -> [[Block; CHUNK_LENGTH]; CHUNK_HEIGHT] {
        let mut blocks = [[Block::default(); CHUNK_LENGTH]; CHUNK_HEIGHT];
        let noisemap = &data.noisemap;
        let treemap = &data.treemap;

        for stage_index in 0..blocks.len() {
            for block_index in 0..blocks[stage_index].len() {
                match stage_index {
                    0 => {
                        if let Some(&noise_value) = noisemap.get(block_index) {
                            blocks[stage_index][block_index] =
                                Block::new(generate_block_type(noise_value));
                        }
                    }
                    1 => {
                        let bottom_block = blocks[0][block_index].get_type();

                        if let Some(&noise_value) = treemap.get(block_index) {
                            if let Some(block_type) =
                                generate_detail_block_type(bottom_block, noise_value)
                            {
                                blocks[stage_index][block_index] = Block::new(block_type);
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        blocks
    }
}

fn generate_block_type(noise_value: f32) -> BlockType {
    match noise_value {
        v if v < 0.45 => BlockType::Water,
        v if v < 0.6 => BlockType::Sand,
        _ => BlockType::Grass,
    }
}

fn generate_detail_block_type(bottom_block: &BlockType, noise_value: f32) -> Option<BlockType> {
    match bottom_block {
        &BlockType::Grass => match noise_value {
            v if v > 0.9 => Some(BlockType::DeadTree),
            v if v > 0.8 => Some(BlockType::Branch),
            v if v > 0.7 => Some(BlockType::Plant),
            v if v > 0.6 => Some(BlockType::Rock),
            v if v > 0.5 => Some(BlockType::Flower),
            _ => None,
        },
        &BlockType::Sand => match noise_value {
            v if v > 0.9 => Some(BlockType::Pebble),
            _ => None,
        },
        _ => None,
    }
}
