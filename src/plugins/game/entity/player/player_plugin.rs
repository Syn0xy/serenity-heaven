use bevy::prelude::*;

use crate::{
    constants::player_datas,
    models::{
        assets::texture::TextureAssets,
        game::{
            physics::{
                collider::{BoxCollider, Collider, SphereCollider},
                GTransform, Rigidbody,
            },
            player::{Player, PlayerController},
        },
    },
};

use super::PlayerControllerPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerControllerPlugin)
            .add_systems(Startup, (setup_player, setup_test));
    }
}

fn setup_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let player_image = texture_assets
        .get_texture(player_datas::PLAYER_IDLE_ID)
        .unwrap();

    let player_entity = commands.spawn((
        Player,
        PlayerController::default(),
        Rigidbody::new(player_datas::PLAYER_MASS),
        Collider::Sphere(SphereCollider::new(player_datas::PLAYER_RADIUS_COLLIDER)),
        // Collider::Box(BoxCollider::new(1.0, 1.0)),
        GTransform {
            position: Vec2::ZERO,
        },
        SpriteBundle {
            texture: player_image.image.clone(),
            ..default()
        },
    ));

    println!("Player entity: {:?}", player_entity.id());
}

fn setup_test(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let player_image = texture_assets
        .get_texture(player_datas::PLAYER_IDLE_ID)
        .unwrap();

    for i in 0..1 {
        let delta = Vec2::splat(i as f32 / 50.0);
        let sphere_entity = commands.spawn((
            Collider::Sphere(SphereCollider::new(1.0)),
            GTransform {
                position: Vec2::new(5.0, 0.0) + delta,
            },
            SpriteBundle {
                texture: player_image.image.clone(),
                ..default()
            },
        ));

        println!("Sphere entity: {:?}", sphere_entity.id());
    }

    commands.spawn((
        Collider::Box(BoxCollider::new(2.0, 5.0)),
        GTransform {
            position: Vec2::new(-5.0, 0.0),
        },
        SpriteBundle {
            texture: player_image.image.clone(),
            ..default()
        },
    ));
}
