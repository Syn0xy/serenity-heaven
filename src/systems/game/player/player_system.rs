use bevy::prelude::*;

use crate::{
    components::{
        physics::{
            collider::{Collider, SphereCollider},
            GTransform, Rigidbody,
        },
        player::{Player, PlayerController},
    },
    constants::player_datas,
    resources::TextureAssets,
};

pub fn setup_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let player_image = texture_assets
        .get_texture(player_datas::PLAYER_IDLE_ID)
        .unwrap();

    commands.spawn((
        Player,
        PlayerController::default(),
        Rigidbody::new(player_datas::PLAYER_MASS, player_datas::PLAYER_DRAG),
        Collider::Sphere(SphereCollider::new(player_datas::PLAYER_RADIUS_COLLIDER)),
        GTransform {
            position: Vec2::ZERO,
        },
        SpriteBundle {
            texture: player_image.image.clone(),
            ..default()
        },
    ));

    commands.spawn((
        Collider::Sphere(SphereCollider::new(2.0)),
        GTransform {
            position: Vec2::new(5.0, 0.0),
        },
        SpriteBundle {
            texture: player_image.image.clone(),
            ..default()
        },
    ));
}
