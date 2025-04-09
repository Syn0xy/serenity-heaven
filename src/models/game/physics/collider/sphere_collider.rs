use bevy::math::Vec2;

use super::{collides_box_and_sphere, BoxCollider, Collision, CollisionDetection};

#[derive(Default, Debug)]
pub struct SphereCollider {
    pub radius: f32,
    _radius_sqr: f32,
}

impl SphereCollider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            _radius_sqr: radius * radius,
        }
    }
}

impl CollisionDetection for SphereCollider {
    fn collides_with_sphere(
        &self,
        sphere_collider: &SphereCollider,
        position: Vec2,
        other_position: Vec2,
    ) -> Option<Collision> {
        let total_radius = self.radius + sphere_collider.radius;
        let total_radius_sqr = total_radius * total_radius;
        let delta = position - other_position;
        let dist_sqr = delta.length_squared();
        let penetration_depth_sqr = total_radius_sqr - dist_sqr;

        if penetration_depth_sqr <= 0.0 {
            return None;
        }

        // let penetration_depth = total_radius - position.distance(other_position);

        let dir = delta.normalize_or_zero();
        let penetration = dir * (penetration_depth_sqr / (total_radius * 2.0));

        Some(Collision { penetration })
    }

    fn collides_with_box(
        &self,
        box_collider: &BoxCollider,
        sphere_position: Vec2,
        box_position: Vec2,
    ) -> Option<Collision> {
        collides_box_and_sphere(box_collider, self, box_position, sphere_position)
    }
}
