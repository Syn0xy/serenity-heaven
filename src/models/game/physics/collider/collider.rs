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
    let half_width = box_collider.width * 0.5;
    let half_height = box_collider.height * 0.5;
    let min_x = sphere_position.x - half_width;
    let min_y = sphere_position.y - half_height;
    let max_x = sphere_position.x + half_width;
    let max_y = sphere_position.y + half_height;

    let closest_point = Vec2::new(
        box_position.x.clamp(min_x, max_x),
        box_position.y.clamp(min_y, max_y),
    );
    let dist = box_position.distance(closest_point);
    let delta = sphere_collider.radius - dist;

    if delta > 0.0 {
        let total_width = half_width + sphere_collider.radius;
        let total_height = half_height + sphere_collider.radius;
        let dist_x = box_position.x - sphere_position.x;
        let dist_y = box_position.y - sphere_position.y;
        let dist_x_abs = dist_x.abs();
        let dist_y_abs = dist_y.abs();
        let delta_x = total_width - dist_x_abs;
        let delta_y = total_height - dist_y_abs;

        let penetration = if delta_x < delta_y {
            Vec2::new(delta_x.copysign(dist_x), 0.0)
        } else {
            Vec2::new(0.0, delta_y.copysign(dist_y))
        };

        Some(Collision { penetration })
    } else {
        None
    }
}
