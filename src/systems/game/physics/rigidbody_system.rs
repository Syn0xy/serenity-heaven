use bevy::{
    ecs::system::{Query, Res},
    time::Time,
};

use crate::components::physics::{GTransform, Rigidbody};

pub fn update_rigidbodies(
    mut rigidbody_query: Query<(&mut GTransform, &mut Rigidbody)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut gtransform, mut rigidbody) in rigidbody_query.iter_mut() {
        rigidbody.update(delta_time);
        gtransform.position += rigidbody.velocity;
    }
}
