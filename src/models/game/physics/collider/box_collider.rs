use bevy::math::Vec2;

use super::{collides_box_and_sphere, Collision, CollisionDetection, SphereCollider};

#[derive(Default, Debug)]
pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
}

impl BoxCollider {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl CollisionDetection for BoxCollider {
    fn collides_with_sphere(
        &self,
        sphere_collider: &SphereCollider,
        box_position: Vec2,
        sphere_position: Vec2,
    ) -> Option<Collision> {
        collides_box_and_sphere(self, sphere_collider, box_position, sphere_position)
    }

    fn collides_with_box(
        &self,
        box_collider: &BoxCollider,
        position: Vec2,
        other_position: Vec2,
    ) -> Option<Collision> {
        let half_total_width = (self.width + box_collider.width) * 0.5;
        let half_total_height = (self.height + box_collider.height) * 0.5;
        let dist_x = position.x - other_position.x;
        let dist_y = position.y - other_position.y;
        let delta_x = half_total_width - dist_x.abs();
        let delta_y = half_total_height - dist_y.abs();

        if delta_x <= 0.0 || delta_y <= 0.0 {
            return None;
        }

        let penetration = if delta_x > delta_y {
            Vec2::new(0.0, delta_y.copysign(dist_y))
        } else {
            Vec2::new(delta_x.copysign(dist_x), 0.0)
        };

        Some(Collision { penetration })
    }
}
