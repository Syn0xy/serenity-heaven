use bevy::{
    app::{App, Plugin, Update},
    ecs::{entity::Entity, system::Query},
};

use crate::models::game::physics::{
    collider::{Collider, Collision},
    GTransform,
};

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_colliders);
    }
}

fn update_colliders(mut collider_query: Query<(Entity, &mut GTransform, &Collider)>) {
    let mut collisions: Vec<(Entity, Entity, Collision)> = Vec::new();
    let colliders: Vec<_> = collider_query.iter().collect();

    for &(e1, gt1, c1) in &colliders {
        for &(e2, gt2, c2) in &colliders {
            if e1 <= e2 {
                continue;
            }

            c1.check_collision(c2, gt1.position, gt2.position)
                .map(|collision| collisions.push((e1, e2, collision)));
        }
    }

    for (e1, e2, collision) in &collisions {
        println!(
            "Collision between {:?} : {:?}   ;   {:.4?}",
            e1.index(),
            e2.index(),
            collision.penetration
        );

        if let Ok((_, mut gt, _)) = collider_query.get_mut(*e1) {
            gt.position += collision.penetration / 2.0;
        }

        if let Ok((_, mut gt, _)) = collider_query.get_mut(*e2) {
            gt.position -= collision.penetration / 2.0;
        }
    }
}
