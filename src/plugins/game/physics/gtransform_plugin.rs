use bevy::prelude::*;

use crate::{
    constants::texture_datas,
    models::{
        display::Resolution,
        game::{physics::GTransform, player::Player},
    },
};

pub struct GTransformPlugin;

impl Plugin for GTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_gtransforms);
    }
}

fn update_gtransforms(
    mut transforms_query: Query<(&mut Transform, &GTransform)>,
    player_query: Query<&GTransform, With<Player>>,
    resolution: Res<Resolution>,
) {
    if let Ok(player_gtransform) = player_query.get_single() {
        let transform_scale = Vec3::splat(resolution.pixel_ratio);
        let scale = texture_datas::TILE_SIZE_F32 * resolution.pixel_ratio;
        let player_position = player_gtransform.position;

        for (mut transform, gtransform) in transforms_query.iter_mut() {
            let delta_position = gtransform.position - player_position;
            let world_position = Vec3::new(delta_position.x, delta_position.y, 0.0) * scale;
            transform.translation = world_position.round();
            transform.scale = transform_scale;
        }
    }
}
