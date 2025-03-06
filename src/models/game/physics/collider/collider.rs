use bevy::prelude::*;

use super::{BoxCollider, SphereCollider};

#[derive(Component, Debug)]
pub enum Collider {
    Box(BoxCollider),
    Sphere(SphereCollider),
}

#[derive(Debug)]
pub struct Collision {
    // pub entity_a: Entity,
    // pub entity_b: Entity,
    pub penetration: Vec2,
}

impl Collider {
    pub fn check_collision(
        &self,
        other: &Collider,
        position: Vec2,
        other_position: Vec2,
    ) -> Option<Collision> {
        match (self, other) {
            (Collider::Box(a), Collider::Box(b)) => {
                a.collides_with_box(b, position, other_position)
            }
            (Collider::Box(a), Collider::Sphere(b)) => {
                a.collides_with_sphere(b, position, other_position)
            }
            (Collider::Sphere(a), Collider::Box(b)) => {
                a.collides_with_box(b, position, other_position)
            }
            (Collider::Sphere(a), Collider::Sphere(b)) => {
                a.collides_with_sphere(b, position, other_position)
            }
        }
    }
}

pub trait CollisionDetection {
    fn collides_with_sphere(
        &self,
        sphere_collider: &SphereCollider,
        position: Vec2,
        other_position: Vec2,
    ) -> Option<Collision>;
    fn collides_with_box(
        &self,
        box_collider: &BoxCollider,
        position: Vec2,
        other_position: Vec2,
    ) -> Option<Collision>;
}

pub(super) fn collides_box_and_sphere(
    box_collider: &BoxCollider,
    sphere_collider: &SphereCollider,
    box_position: Vec2,
    sphere_position: Vec2,
) -> Option<Collision> {
    let half_extents = Vec2::new(box_collider.width / 2.0, box_collider.height / 2.0);
    let min = sphere_position - half_extents;
    let max = sphere_position + half_extents;

    let closest_point = Vec2::new(
        box_position.x.clamp(min.x, max.x),
        box_position.y.clamp(min.y, max.y),
    );
    let dist = box_position.distance(closest_point);

    if dist <= sphere_collider.radius {
        Some(Collision {
            penetration: Vec2::new(0.0, 0.0),
        })
    } else {
        None
    }
}
