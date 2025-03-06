use crate::models::assets::texture::PlayerId;

pub const PLAYER_IDLE_ID: PlayerId = PlayerId::Test;

pub const PLAYER_MAX_SPEED: f32 = 1.0 / 15.0;
pub const PLAYER_MAX_ACCEL: f32 = 30.0 * PLAYER_MAX_SPEED;
pub const PLAYER_MASS: f32 = 1.0;
pub const PLAYER_DRAG: f32 = 8.0;
pub const PLAYER_RADIUS_COLLIDER: f32 = 0.5;
