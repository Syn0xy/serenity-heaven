use bevy::{
    app::{App, Plugin, Update},
    ecs::{entity::Entity, system::Query},
};

use crate::components::physics::{collider::Collider, ForceMode, GTransform, Rigidbody};

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_colliders);
    }
}

fn update_colliders(
    collider_query: Query<(Entity, &GTransform, &Collider)>,
    mut rigidbody_query: Query<&mut Rigidbody>,
) {
    let mut collisions = Vec::new();

    for (e1, gt1, c1) in collider_query.iter() {
        for (e2, gt2, c2) in collider_query.iter() {
            if e1 <= e2 {
                continue;
            }

            if c1.check_collision(c2, &gt1.position, &gt2.position) {
                collisions.push((e1, gt1, c1, e2, gt2, c2));
            }
        }
    }

    for (e1, gt1, c1, e2, gt2, c2) in collisions {
        println!("Collision between {:?} ; {:?}", e1, e2);

        let c1_radius = match c1 {
            Collider::Sphere(s) => s.radius,
            _ => 0.0,
        };
        let c2_radius = match c2 {
            Collider::Sphere(s) => s.radius,
            _ => 0.0,
        };

        let total = c1_radius + c2_radius;
        let dist = gt1.position.distance(gt2.position);

        let dir = (gt1.position - gt2.position).normalize() * (total - dist) / 2.0;
        rigidbody_query.get_mut(e1).iter_mut().for_each(|rgdb| {
            rgdb.add_force(dir, ForceMode::Force);
        });
        rigidbody_query.get_mut(e2).iter_mut().for_each(|rgdb| {
            rgdb.add_force(-dir, ForceMode::Force);
        });
    }
}
