use perlin_noise::{NoiseDescriptor, NormalizeMode};

use crate::constants::chunk_datas;

pub const TERRAIN_NORMALIZE_MODE: NormalizeMode = NormalizeMode::Global;
pub const TERRAIN_DESCRIPTOR: NoiseDescriptor = NoiseDescriptor {
    seed: 2,
    width: chunk_datas::CHUNK_SIZE,
    height: chunk_datas::CHUNK_SIZE,
    scale: 80.0,
    octaves: 2,
    persistance: 0.5,
    lacunarity: 2.0,
};

pub const TREE_NORMALIZE_MODE: NormalizeMode = NormalizeMode::Local;
pub const TREE_DESCRIPTION: NoiseDescriptor = NoiseDescriptor {
    scale: 1.0,
    octaves: 1,
    persistance: 1.0,
    lacunarity: 1.0,
    ..TERRAIN_DESCRIPTOR
};
