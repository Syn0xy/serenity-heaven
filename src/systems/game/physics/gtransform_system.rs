use bevy::prelude::*;

use crate::{
    components::{physics::GTransform, player::Player},
    resources::Resolution,
};

pub fn update_gtransforms(
    mut entities_query: Query<(&GTransform, &mut Transform)>,
    player_query: Query<&GTransform, With<Player>>,
    resolution: Res<Resolution>,
) {
    if player_query.is_empty() {
        return;
    }

    let resolution_scale = Vec3::splat(resolution.pixel_ratio);
    let player_transform = player_query.single();
    let player_world_position = player_transform.to_transform_position();

    for (entity_transform, mut transform) in entities_query.iter_mut() {
        let scale = resolution_scale.clone();
        let entity_world_position = entity_transform.to_transform_position();
        transform.translation = (entity_world_position - player_world_position) * scale;
        transform.scale = scale;
    }
}
