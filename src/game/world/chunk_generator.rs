use bevy::math::Vec2;
use perlin_noise::NoiseOffset;

use super::{chunk::Chunk, noise_descriptor};

#[derive(Default, Debug)]
pub struct ChunkGenerator;

#[derive(Debug)]
pub struct ChunkData {
    pub offset: NoiseOffset,
    pub noisemap: Vec<f32>,
    pub treemap: Vec<f32>,
}

impl ChunkGenerator {
    pub fn generate_data(&self, offset: NoiseOffset) -> ChunkData {
        let noisemap = perlin_noise::generate_map(
            &noise_descriptor::TERRAIN_DESCRIPTOR,
            &offset,
            &noise_descriptor::TERRAIN_NORMALIZE_MODE,
        );

        let treemap = perlin_noise::generate_map(
            &noise_descriptor::TREE_DESCRIPTION,
            &offset,
            &noise_descriptor::TREE_NORMALIZE_MODE,
        );

        ChunkData {
            offset,
            noisemap,
            treemap,
        }
    }

    pub fn generate(&self, position: &Vec2) -> Chunk {
        Chunk::new(self.generate_data(NoiseOffset::from(position.to_array())))
    }
}
