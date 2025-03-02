use bevy::app::{App, Plugin, PostUpdate};

use crate::systems::game::physics;

pub struct GTransformPlugin;

impl Plugin for GTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, physics::update_gtransforms);
    }
}
