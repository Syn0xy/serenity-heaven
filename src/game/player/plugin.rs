use bevy::prelude::*;

use crate::{
    game::{rigidbody::Rigidbody, GTransform},
    loader::texture::{resource::TextureAssets, texture_id::*},
};

use super::{
    controller::{PlayerController, PlayerControllerPlugin},
    Player,
};

const PLAYER_IDLE_ID: PlayerId = PlayerId::Test;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerControllerPlugin)
            .add_systems(Startup, setup_player);
    }
}

fn setup_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let player_image = texture_assets.get_texture(PLAYER_IDLE_ID).unwrap();

    commands.spawn((
        Player,
        PlayerController::default(),
        Rigidbody::new(1.0),
        GTransform {
            position: Vec2::ZERO,
        },
        SpriteBundle {
            texture: player_image.image.clone(),
            ..default()
        },
    ));
}
