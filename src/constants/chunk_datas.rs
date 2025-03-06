use crate::models::assets::texture::TilesetId;

pub const TILESET_DETAIL_ID: TilesetId = TilesetId::Detail;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;

pub const HALF_CHUNK_SIZE: usize = CHUNK_SIZE / 2;
pub const HALF_CHUNK_SIZE_F32: f32 = HALF_CHUNK_SIZE as f32;

pub const CHUNK_HEIGHT: usize = 2;
pub const CHUNK_LENGTH: usize = CHUNK_SIZE * CHUNK_SIZE;

pub const VIEWER_MOVE_UPDATE_THRESHOLD: f32 = CHUNK_SIZE as f32 / 4.;
pub const CHUNK_VIEW_RADIUS: i32 = CHUNK_SIZE as i32 * 4;
pub const CHUNK_VIEW_RADIUS_SQ: f32 = (CHUNK_VIEW_RADIUS * CHUNK_VIEW_RADIUS) as f32;
