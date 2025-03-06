use bevy::math::Vec2;

use crate::constants::chunk_datas::CHUNK_SIZE_F32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for ChunkCoord {
    fn from(c: (i32, i32)) -> Self {
        let (x, y) = c;
        Self { x, y }
    }
}

impl From<[i32; 2]> for ChunkCoord {
    fn from(a: [i32; 2]) -> Self {
        let [x, y] = a;
        Self { x, y }
    }
}

impl From<&Vec2> for ChunkCoord {
    fn from(v: &Vec2) -> Self {
        Self {
            x: (v.x / CHUNK_SIZE_F32).round() as i32,
            y: (v.y / CHUNK_SIZE_F32).round() as i32,
        }
    }
}

impl From<&ChunkCoord> for Vec2 {
    fn from(c: &ChunkCoord) -> Self {
        Self {
            x: c.x as f32 * CHUNK_SIZE_F32,
            y: c.y as f32 * CHUNK_SIZE_F32,
        }
    }
}
