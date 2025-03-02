use perlin_noise::NoiseOffset;

#[derive(Debug)]
pub struct ChunkData {
    pub offset: NoiseOffset,
    pub noisemap: Vec<f32>,
    pub treemap: Vec<f32>,
}
