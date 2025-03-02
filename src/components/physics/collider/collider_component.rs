use bevy::prelude::*;

use super::{BoxCollider, SphereCollider};

#[derive(Component, Debug)]
pub enum Collider {
    Box(BoxCollider),
    Sphere(SphereCollider),
}

impl Collider {
    pub fn check_collision(
        &self,
        other: &Collider,
        position: &Vec2,
        other_position: &Vec2,
    ) -> bool {
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
        position: &Vec2,
        other_position: &Vec2,
    ) -> bool;
    fn collides_with_box(
        &self,
        box_collider: &BoxCollider,
        position: &Vec2,
        other_position: &Vec2,
    ) -> bool;
}
