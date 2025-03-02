use bevy::math::Vec2;
use perlin_noise::{NoiseDescriptor, NoiseOffset, NormalizeMode};

use crate::{
    components::world::Chunk, constants::noise_descriptor, models::world::chunk_data::ChunkData,
};

#[derive(Debug)]
pub struct ChunkGenerator {
    pub noisemap_descriptor: NoiseDescriptor,
    pub noisemap_normalize: NormalizeMode,

    pub treemap_descriptor: NoiseDescriptor,
    pub treemap_normalize: NormalizeMode,
}

impl Default for ChunkGenerator {
    fn default() -> Self {
        Self {
            noisemap_descriptor: noise_descriptor::TERRAIN_DESCRIPTOR,
            noisemap_normalize: noise_descriptor::TERRAIN_NORMALIZE_MODE,

            treemap_descriptor: noise_descriptor::TREE_DESCRIPTION,
            treemap_normalize: noise_descriptor::TREE_NORMALIZE_MODE,
        }
    }
}

impl ChunkGenerator {
    pub fn generate_data(&self, offset: NoiseOffset) -> ChunkData {
        let noisemap = perlin_noise::generate_map(
            &self.noisemap_descriptor,
            &offset,
            &self.noisemap_normalize,
        );

        let treemap =
            perlin_noise::generate_map(&self.treemap_descriptor, &offset, &self.treemap_normalize);

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
