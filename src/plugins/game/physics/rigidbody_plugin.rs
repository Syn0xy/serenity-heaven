use bevy::app::{App, Plugin, Update};

use crate::systems::game::physics;

pub struct RigidbodyPlugin;

impl Plugin for RigidbodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, physics::update_rigidbodies);
    }
}
