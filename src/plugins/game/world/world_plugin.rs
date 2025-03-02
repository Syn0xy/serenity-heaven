use bevy::prelude::*;

use crate::{resources, systems::game::world};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::World::default())
            .add_systems(Update, world::update_world);
    }
}
