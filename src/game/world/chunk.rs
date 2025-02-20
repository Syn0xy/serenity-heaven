use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    render::view::VisibilityBundle,
    sprite::{SpriteBundle, TextureAtlas},
    transform::components::{GlobalTransform, Transform},
    utils::default,
};

use crate::{
    game::GTransform,
    loader::texture::{resource::TextureAssets, texture_id::*},
};

use super::{
    block::Block,
    block_type::BlockType,
    chunk_coord::ChunkCoord,
    chunk_generator::{ChunkData, ChunkGenerator},
};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 1;
pub const CHUNK_TOTAL_LENGTH: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;
pub const HALF_CHUNK_SIZE: usize = CHUNK_SIZE / 2;
pub const HALF_CHUNK_SIZE_F32: f32 = HALF_CHUNK_SIZE as f32;

const TILESET_GRASS_ID: TilesetId = TilesetId::Grass;

#[derive(Component)]
pub struct Chunk {
    _data: ChunkData,
    blocks: [Block; CHUNK_TOTAL_LENGTH],
}

impl Chunk {
    pub fn new(_data: ChunkData) -> Self {
        let blocks = Chunk::generate_blocks(&_data);
        Self { _data, blocks }
    }

    pub fn generate_blocks(data: &ChunkData) -> [Block; CHUNK_TOTAL_LENGTH] {
        let mut blocks = [Block::default(); CHUNK_TOTAL_LENGTH];
        let noisemap = &data.noisemap;

        for block_index in 0..blocks.len() {
            if let Some(&noise_value) = noisemap.get(block_index) {
                blocks[block_index] = Block::new(generate_block_type(noise_value));
            }
        }

        blocks
    }
}

fn generate_block_type(noise_value: f32) -> BlockType {
    if noise_value < 0.45 {
        BlockType::Water
    }
    // else if noise_value < 0.55 {
    //     BlockType::Sand
    // }
    else {
        BlockType::Grass
    }
}

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
        chunk_entity,
        texture_assets,
        &position,
        &generator,
    );

    chunk_entity
}

fn spawn_tiles(
    commands: &mut Commands,
    chunk_entity: Entity,
    texture_assets: &Res<TextureAssets>,
    chunk_position: &Vec2,
    generator: &ChunkGenerator,
) {
    let atlas_asset = texture_assets.get_atlas(TILESET_GRASS_ID).unwrap();
    let animation_sheet = &atlas_asset.image;
    let texture_atlas_handle = &atlas_asset.layout;
    let pixel_size = atlas_asset.pixel_size as f32;
    let chunk = generator.generate(chunk_position);

    for (block_index, block) in chunk.blocks.iter().enumerate() {
        let x = block_index / CHUNK_SIZE;
        let y = block_index % CHUNK_SIZE;

        let (up_index, right_index, diag_index) = get_corner_blocks(x, y, block_index);

        let crnt_type = block.get_type();
        let up_type = chunk.blocks[up_index].get_type();
        let diag_type = chunk.blocks[diag_index].get_type();
        let right_type = chunk.blocks[right_index].get_type();

        let atlas_code = get_atlas_code(
            &BlockType::Grass,
            (diag_type, up_type, right_type, crnt_type),
        );

        let block_position =
            Vec3::new(x as f32 + 0.5, (CHUNK_SIZE - y) as f32 - 0.5, -1.0) * pixel_size;
        let block_entity = commands
            .spawn((
                SpriteBundle {
                    texture: animation_sheet.clone(),
                    transform: Transform::from_translation(block_position),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_handle.clone(),
                    index: get_atlas_index(atlas_code),
                },
            ))
            .id();

        commands.entity(chunk_entity).add_child(block_entity);
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
