use bevy::{
    app::{App, Plugin, Update},
    ecs::system::{Query, Res},
    time::Time,
};

use crate::game::GTransform;

use super::Rigidbody;

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
        rigidbody.update(&mut gtransform, delta_time);
    }
}
