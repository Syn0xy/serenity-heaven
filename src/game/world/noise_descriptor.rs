use perlin_noise::{NoiseDescriptor, NormalizeMode};

use super::chunk;

pub(super) const TERRAIN_NORMALIZE_MODE: NormalizeMode = NormalizeMode::Global;
pub(super) const TERRAIN_DESCRIPTOR: NoiseDescriptor = NoiseDescriptor {
    seed: 2,
    width: chunk::CHUNK_SIZE,
    height: chunk::CHUNK_SIZE,
    scale: 80.0,
    octaves: 2,
    persistance: 0.5,
    lacunarity: 2.0,
};

pub(super) const TREE_NORMALIZE_MODE: NormalizeMode = NormalizeMode::Local;
pub(super) const TREE_DESCRIPTION: NoiseDescriptor = NoiseDescriptor {
    scale: 1.0,
    octaves: 1,
    persistance: 1.0,
    lacunarity: 1.0,
    ..TERRAIN_DESCRIPTOR
};
