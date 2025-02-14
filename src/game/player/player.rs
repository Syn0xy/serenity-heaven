use bevy::prelude::*;

use crate::{
    game::GTransform,
    loader::{assets::TextureAssets, datas::asset_name},
};

use super::PlayerControllerPlugin;

const PLAYER_IDLE: usize = asset_name::player::IDLE;

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
    let player_image = texture_assets.get_texture(&PLAYER_IDLE).unwrap();

    commands.spawn((
        Player,
        GTransform {
            position: Vec2::ZERO,
        },
        SpriteBundle {
            texture: player_image.image.clone(),
            ..default()
        },
    ));
}
