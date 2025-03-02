use bevy::prelude::*;

use crate::{
    components::{physics::GTransform, player::Player},
    constants::chunk_datas::*,
    models::world::chunk_coord::ChunkCoord,
    resources::{TextureAssets, World},
};

use super::chunk_system;

pub fn update_world(
    mut commands: Commands,
    mut world: ResMut<World>,
    texture_assets: Res<TextureAssets>,
    player_query: Query<&GTransform, With<Player>>,
) {
    if let Some(player_transform) = player_query.get_single().ok() {
        let player_position = player_transform.position;

        if world.last_viewer_position.map_or(true, |old_position| {
            player_position.distance(old_position) >= VIEWER_MOVE_UPDATE_THRESHOLD
        }) {
            world.last_viewer_position.replace(player_position);
            update_visible_chunks(&mut world, &mut commands, &texture_assets, &player_position);
        }
    }
}

fn chunk_is_visible(viewer_position: &Vec2, chunk_position: &Vec2) -> bool {
    (*viewer_position).distance_squared(*chunk_position + HALF_CHUNK_SIZE_F32)
        <= CHUNK_VIEW_RADIUS_SQ
}

fn unvisible_far_chunks(world: &mut World, commands: &mut Commands, viewer_position: &Vec2) {
    world.visible_chunks.retain(|coord| {
        let chunk_position = &Vec2::from(coord);

        if let Some(&chunk_entity) = world.chunks.get(coord) {
            let chunk_is_visible = chunk_is_visible(viewer_position, chunk_position);

            commands.entity(chunk_entity).insert(if chunk_is_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            });

            chunk_is_visible
        } else {
            false
        }
    });
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
        world.visible_chunks.push(*chunk_coord);
    } else {
        if let Some(&entity) = world.chunks.get(chunk_coord) {
            commands.entity(entity).despawn_recursive();
            world.chunks.remove(chunk_coord);
        }
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
            chunk_system::spawn_chunk(commands, &world.generator, chunk_coord, texture_assets);

        world.chunks.insert(*chunk_coord, chunk_entity);
    }
}
