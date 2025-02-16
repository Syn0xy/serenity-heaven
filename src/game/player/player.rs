use bevy::prelude::*;

use crate::{
    game::GTransform,
    loader::{
        texture::{resource::TextureAssets, texture_id::*},
        AssetId::{self, *},
    },
};

use super::{PlayerController, PlayerControllerPlugin};

const PLAYER_IDLE_ID: &AssetId = &Texture(Static(StaticId::Player(PlayerId::Idle)));

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerControllerPlugin)
            .add_systems(Startup, setup_player);
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let player_image = texture_assets.get_texture(PLAYER_IDLE_ID).unwrap();

    commands.spawn((
        Player,
        PlayerController::default(),
        GTransform {
            position: Vec2::ZERO,
        },
        SpriteBundle {
            texture: player_image.image.clone(),
            ..default()
        },
    ));
}
