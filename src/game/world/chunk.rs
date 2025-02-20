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
    loader::{
        self,
        texture::{resource::TextureAssets, texture_id::*},
    },
};

use super::{
    block::Block,
    block_type::BlockType,
    chunk_coord::ChunkCoord,
    chunk_generator::{ChunkData, ChunkGenerator},
};

pub const CHUNK_SIZE: usize = 4;
pub const CHUNK_HEIGHT: usize = 1;
pub const CHUNK_TOTAL_LENGTH: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

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
    } else if noise_value < 0.55 {
        BlockType::Sand
    } else {
        BlockType::Grass
    }
}

pub fn spawn_chunk(
    commands: &mut Commands,
    chunk_data_generator: &ChunkGenerator,
    chunk_coord: &ChunkCoord,
    texture_assets: &Res<TextureAssets>,
) -> Entity {
    let position = Vec2::from(chunk_coord);
    let chunk = chunk_data_generator.generate(&position);

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
        &chunk_data_generator,
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

    let chunk = generator.generate(chunk_position);

    chunk
        .blocks
        .iter()
        .enumerate()
        .for_each(|(block_index, block)| {
            let x = (block_index / CHUNK_SIZE) as f32;
            let y = (CHUNK_SIZE - block_index % CHUNK_SIZE) as f32;

            let block_position = Vec3::new(x, y, -1.0) * loader::texture::TILE_SIZE as f32;
            let block_entity = commands
                .spawn((
                    SpriteBundle {
                        texture: animation_sheet.clone(),
                        transform: Transform::from_translation(block_position),
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_handle.clone(),
                        index: block.get_atlas_index(),
                    },
                ))
                .id();

            commands.entity(chunk_entity).add_child(block_entity);
        });
}
