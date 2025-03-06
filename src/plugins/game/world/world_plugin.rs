use bevy::prelude::*;

use crate::{
    constants::chunk_datas::{
        CHUNK_HEIGHT, CHUNK_LENGTH, CHUNK_SIZE, CHUNK_VIEW_RADIUS, CHUNK_VIEW_RADIUS_SQ,
        HALF_CHUNK_SIZE_F32, TILESET_DETAIL_ID, VIEWER_MOVE_UPDATE_THRESHOLD,
    },
    models::{
        assets::texture::{AtlasAsset, TextureAssets},
        game::{
            physics::GTransform,
            player::Player,
            world::{
                chunk::{Block, BlockType, ChunkCoord, ChunkGenerator},
                World,
            },
        },
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default())
            .add_systems(Update, update_world);
    }
}

fn update_world(
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
        let chunk_entity = spawn_chunk(commands, &world.generator, chunk_coord, texture_assets);

        world.chunks.insert(*chunk_coord, chunk_entity);
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    generator: &ChunkGenerator,
    chunk_coord: &ChunkCoord,
    texture_assets: &Res<TextureAssets>,
) -> Entity {
    let position = Vec2::from(chunk_coord);
    let chunk = generator.generate(&position);

    let chunk_entity = commands
        .spawn((
            chunk,
            GTransform { position },
            Transform::default(),
            GlobalTransform::default(),
            VisibilityBundle::default(),
        ))
        .id();

    spawn_tiles(
        commands,
        texture_assets,
        chunk_entity,
        &position,
        &generator,
    );

    chunk_entity
}

fn spawn_tiles(
    commands: &mut Commands,
    texture_assets: &Res<TextureAssets>,
    chunk_entity: Entity,
    chunk_position: &Vec2,
    generator: &ChunkGenerator,
) {
    let chunk = generator.generate(chunk_position);

    for (stage_index, stage_blocks) in chunk.blocks.iter().enumerate() {
        for (block_index, block) in stage_blocks.iter().enumerate() {
            if block.is_air() {
                continue;
            }

            let z = (CHUNK_HEIGHT - stage_index) * 2;
            let x = block_index / CHUNK_SIZE;
            let y = block_index % CHUNK_SIZE;

            let back_position = (x, y, z + 1);
            let fore_position = (x, y, z);

            let atlas_asset = texture_assets.get_atlas(TILESET_DETAIL_ID).unwrap();
            let atlas_index = block.get_atlas_index();

            spawn_tile(
                commands,
                chunk_entity,
                back_position,
                atlas_index,
                &atlas_asset,
            );
            check_connected_blocks(
                commands,
                chunk_entity,
                texture_assets,
                stage_blocks,
                block,
                fore_position,
                block_index,
            );
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    chunk_entity: Entity,
    position: (usize, usize, usize),
    atlas_index: usize,
    atlas_asset: &AtlasAsset,
) {
    let (x, y, z) = position;
    let animation_sheet = &atlas_asset.image;
    let texture_atlas_handle = &atlas_asset.layout;
    let pixel_size = atlas_asset.pixel_size as f32;

    let block_position = Vec3::new(
        x as f32 + 0.5,
        (CHUNK_SIZE - y) as f32 - 0.5,
        -1.0 - z as f32,
    ) * pixel_size;
    let block_entity = commands
        .spawn((
            SpriteBundle {
                texture: animation_sheet.clone(),
                transform: Transform::from_translation(block_position),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_handle.clone(),
                index: atlas_index,
            },
        ))
        .id();

    commands.entity(chunk_entity).add_child(block_entity);
}

fn check_connected_blocks(
    commands: &mut Commands,
    chunk_entity: Entity,
    texture_assets: &Res<TextureAssets>,
    blocks: &[Block; CHUNK_LENGTH],
    block: &Block,
    position: (usize, usize, usize),
    block_index: usize,
) {
    if let Some(tileset_id) = block.get_atlas_id() {
        let atlas_asset = texture_assets.get_atlas(tileset_id).unwrap();

        let (up_index, right_index, diag_index) =
            get_corner_blocks(position.0, position.1, block_index);

        let crnt_type = block.get_type();
        let up_type = blocks[up_index].get_type();
        let diag_type = blocks[diag_index].get_type();
        let right_type = blocks[right_index].get_type();

        let atlas_code = get_atlas_code(
            &BlockType::Grass,
            (diag_type, up_type, right_type, crnt_type),
        );

        spawn_tile(
            commands,
            chunk_entity,
            position,
            get_atlas_index(atlas_code),
            atlas_asset,
        );
    }
}

const fn get_corner_blocks(x: usize, y: usize, block_index: usize) -> (usize, usize, usize) {
    let up_index = if y != CHUNK_SIZE - 1 {
        block_index + 1
    } else {
        block_index - 1
    };

    let right_index = if x != CHUNK_SIZE - 1 {
        block_index + CHUNK_SIZE
    } else {
        block_index - CHUNK_SIZE
    };

    let diag_index = if y == CHUNK_SIZE - 1 {
        right_index - 1
    } else {
        right_index + 1
    };

    (up_index, right_index, diag_index)
}

fn get_atlas_code(
    research: &BlockType,
    blocks: (&BlockType, &BlockType, &BlockType, &BlockType),
) -> usize {
    let code_0001 = if blocks.0 == research { 0b0001 } else { 0 };
    let code_0010 = if blocks.1 == research { 0b0010 } else { 0 };
    let code_0100 = if blocks.2 == research { 0b0100 } else { 0 };
    let code_1000 = if blocks.3 == research { 0b1000 } else { 0 };

    code_0001 | code_0010 | code_0100 | code_1000
}

const fn get_atlas_index(atlas_code: usize) -> usize {
    match atlas_code {
        0 => 12,
        1 => 13,
        2 => 0,
        3 => 3,
        4 => 8,
        5 => 1,
        6 => 14,
        7 => 5,
        8 => 15,
        9 => 4,
        10 => 11,
        11 => 2,
        12 => 9,
        13 => 10,
        14 => 7,
        15 => 6,
        _ => 0,
    }
}
