use bevy::math::Vec2;
use perlin_noise::{generate_map, NoiseDescriptor, NoiseOffset, NormalizeMode};

use super::chunk::{self, Chunk};

const NORMALIZE_MODE: NormalizeMode = NormalizeMode::Global;
const NOISE_DESCRIPTOR: NoiseDescriptor = NoiseDescriptor {
    seed: 0,
    width: chunk::CHUNK_SIZE,
    height: chunk::CHUNK_SIZE,
    scale: 50.0,
    octaves: 2,
    persistance: 0.5,
    lacunarity: 2.0,
};

#[derive(Debug)]
pub struct ChunkGenerator {
    descriptor: NoiseDescriptor,
    normalize_mode: NormalizeMode,
}

#[derive(Debug)]
pub struct ChunkData {
    pub offset: NoiseOffset,
    pub noisemap: Vec<f32>,
}

impl Default for ChunkGenerator {
    fn default() -> Self {
        Self {
            descriptor: NOISE_DESCRIPTOR,
            normalize_mode: NORMALIZE_MODE,
        }
    }
}

impl ChunkGenerator {
    pub fn new(descriptor: NoiseDescriptor, normalize_mode: NormalizeMode) -> Self {
        Self {
            descriptor,
            normalize_mode,
        }
    }

    pub fn generate_data(&self, offset: NoiseOffset) -> ChunkData {
        let noisemap = generate_map(&self.descriptor, &offset, &self.normalize_mode);

        ChunkData { offset, noisemap }
    }

    pub fn generate(&self, position: &Vec2) -> Chunk {
        Chunk::new(self.generate_data(NoiseOffset::from(position.to_array())))
    }
}
