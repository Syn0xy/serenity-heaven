use bevy::prelude::*;

use crate::models::game::physics::{GTransform, Rigidbody};

pub struct RigidbodyPlugin;

impl Plugin for RigidbodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rigidbodies);
    }
}

pub fn update_rigidbodies(mut rigidbody_query: Query<(&mut GTransform, &mut Rigidbody)>) {
    for (mut gtransform, mut rigidbody) in rigidbody_query.iter_mut() {
        rigidbody.update();
        gtransform.position += rigidbody.velocity;
    }
}
