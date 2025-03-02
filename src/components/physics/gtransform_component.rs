use bevy::prelude::*;

use crate::constants::texture_datas;

#[derive(Component, Default, Debug)]
pub struct GTransform {
    pub position: Vec2,
}

impl GTransform {
    pub fn to_transform_position(&self) -> Vec3 {
        Vec3::new(
            (self.position.x * texture_datas::TILE_SIZE_F32).round(),
            (self.position.y * texture_datas::TILE_SIZE_F32).round(),
            0.0,
        )
    }
}
