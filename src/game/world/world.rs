use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    game::{player::Player, GTransform},
    loader::texture::resource::TextureAssets,
};

use super::{chunk, chunk_coord::ChunkCoord, chunk_generator::ChunkGenerator};

pub const VIEWER_MOVE_UPDATE_THRESHOLD: f32 = chunk::CHUNK_SIZE as f32 / 4.;
pub const CHUNK_VIEW_RADIUS: i32 = chunk::CHUNK_SIZE as i32 * 8;
pub const CHUNK_VIEW_RADIUS_SQ: f32 = (CHUNK_VIEW_RADIUS * CHUNK_VIEW_RADIUS) as f32;
const HALF_CHUNK_SIZE: f32 = chunk::CHUNK_SIZE as f32 / 2.0;

#[derive(Resource, Default)]
pub struct World {
    chunks: HashMap<ChunkCoord, Entity>,
    visible_chunks: Vec<ChunkCoord>,
    last_viewer_position: Option<Vec2>,
    data_generator: ChunkGenerator,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default())
            .add_systems(Update, update_world);
    }
}

fn chunk_is_visible(viewer_position: &Vec2, chunk_position: &Vec2) -> bool {
    (*viewer_position).distance_squared(*chunk_position + HALF_CHUNK_SIZE) <= CHUNK_VIEW_RADIUS_SQ
}

fn unvisible_far_chunks(world: &mut World, commands: &mut Commands, viewer_position: &Vec2) {
    world.visible_chunks.retain(|coord| {
        let chunk_position = &Vec2::from(coord);

        if let Some(&chunk_entity) = world.chunks.get(coord) {
            if chunk_is_visible(viewer_position, chunk_position) {
                commands.entity(chunk_entity).insert(Visibility::Visible);
                true
            } else {
                commands.entity(chunk_entity).insert(Visibility::Hidden);
                false
            }
        } else {
            false
        }
    });
}

fn update_world(
    mut commands: Commands,
    mut world: ResMut<World>,
    texture_assets: Res<TextureAssets>,
    player_query: Query<&GTransform, With<Player>>,
) {
    let player_transform = player_query.single();
    let player_position = player_transform.position;

    if world.last_viewer_position.map_or(true, |old_position| {
        player_position.distance(old_position) >= VIEWER_MOVE_UPDATE_THRESHOLD
    }) {
        world.last_viewer_position.replace(player_position);
        update_visible_chunks(&mut world, &mut commands, &texture_assets, &player_position);
    }
}

fn update_visible_chunks(
    world: &mut World,
    commands: &mut Commands,
    texture_assets: &Res<TextureAssets>,
    viewer_position: &Vec2,
) {
    unvisible_far_chunks(world, commands, viewer_position);

    let viewer_coord = ChunkCoord::from(viewer_position);

    for ox in -CHUNK_VIEW_RADIUS..=CHUNK_VIEW_RADIUS {
        for oy in -CHUNK_VIEW_RADIUS..=CHUNK_VIEW_RADIUS {
            update_visible_chunk(
                world,
                commands,
                texture_assets,
                viewer_position,
                &ChunkCoord::from((viewer_coord.x + ox, viewer_coord.y + oy)),
            );
        }
    }
}

fn update_visible_chunk(
    world: &mut World,
    commands: &mut Commands,
    texture_assets: &Res<TextureAssets>,
    viewer_position: &Vec2,
    chunk_coord: &ChunkCoord,
) {
    if chunk_is_visible(viewer_position, &chunk_coord.into()) {
        update_chunk(world, commands, chunk_coord, texture_assets);
    }
}

fn update_chunk(
    world: &mut World,
    commands: &mut Commands,
    chunk_coord: &ChunkCoord,
    texture_assets: &Res<TextureAssets>,
) {
    if !world.chunks.contains_key(chunk_coord) {
        let chunk_entity =
            chunk::spawn_chunk(commands, &world.data_generator, chunk_coord, texture_assets);

        world.chunks.insert(*chunk_coord, chunk_entity);
    }
    world.visible_chunks.push(*chunk_coord);
}
