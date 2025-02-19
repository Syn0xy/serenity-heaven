use bevy::{
    app::{App, Plugin, PostUpdate, Startup},
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    math::{Vec2, Vec3},
    prelude::{Camera2dBundle, Commands},
    transform::components::Transform,
};

use crate::{display::resolution::Resolution, loader};

use super::{
    player::{self, Player},
    rigidbody, world,
};

const TILE_SIZE_F32: f32 = loader::texture::TILE_SIZE as f32;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            world::WorldPlugin,
            rigidbody::RigidbodyPlugin,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup_game)
        .add_systems(PostUpdate, update_positions);
    }
}

#[derive(Component)]
pub struct GTransform {
    pub position: Vec2,
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_positions(
    player_query: Query<&GTransform, With<Player>>,
    mut entities_query: Query<(&GTransform, &mut Transform)>,
    resolution: Res<Resolution>,
) {
    if player_query.is_empty() {
        return;
    }

    let resolution_scale = Vec3::splat(resolution.pixel_ratio);
    let player_transform = player_query.single();
    let player_world_position = to_screen_position(&player_transform.position);

    for (entity_transform, mut transform) in entities_query.iter_mut() {
        let scale = resolution_scale.clone();
        let entity_world_position = to_screen_position(&entity_transform.position);
        transform.translation = (entity_world_position - player_world_position) * scale;
        transform.scale = scale;
    }
}

fn to_screen_position(position: &Vec2) -> Vec3 {
    Vec3::new(
        (position.x * TILE_SIZE_F32).round(),
        (position.y * TILE_SIZE_F32).round(),
        0.0,
    )
}
