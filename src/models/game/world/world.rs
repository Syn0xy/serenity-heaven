use std::collections::HashMap;

use bevy::prelude::*;

use super::chunk::{ChunkCoord, ChunkGenerator};

#[derive(Resource, Default)]
pub struct World {
    pub generator: ChunkGenerator,
    pub chunks: HashMap<ChunkCoord, Entity>,
    pub visible_chunks: Vec<ChunkCoord>,
    pub last_viewer_position: Option<Vec2>,
}
