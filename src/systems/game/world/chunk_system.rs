use bevy::prelude::*;

use crate::{
    components::physics::GTransform,
    constants::chunk_datas::*,
    models::world::{
        block::Block, block_type::BlockType, chunk_coord::ChunkCoord,
        chunk_generator::ChunkGenerator,
    },
    resources::{AtlasAsset, TextureAssets},
};

pub fn spawn_chunk(
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
