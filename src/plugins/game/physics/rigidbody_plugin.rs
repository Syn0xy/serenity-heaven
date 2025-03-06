use bevy::prelude::*;

use crate::models::game::physics::{GTransform, Rigidbody};

pub struct RigidbodyPlugin;

impl Plugin for RigidbodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rigidbodies);
    }
}

fn update_rigidbodies(
    mut rigidbody_query: Query<(&mut GTransform, &mut Rigidbody)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut gtransform, mut rigidbody) in rigidbody_query.iter_mut() {
        rigidbody.update(delta_time);
        gtransform.position += rigidbody.velocity;
    }
}
