use bevy::math::{Vec2, VectorSpace};

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
        let dist_x = position.x - other_position.x;
        let dist_y = position.y - other_position.y;
        let dist_x_abs = dist_x.abs();
        let dist_y_abs = dist_y.abs();
        let half_total_width = (self.width + box_collider.width) / 2.0;
        let half_total_height = (self.height + box_collider.height) / 2.0;

        if dist_x_abs <= half_total_width && dist_y_abs <= half_total_height {
            let mut penetration = Vec2::ZERO;
            let delta_x = half_total_width - dist_x;
            let delta_y = half_total_height - dist_y;

            if position.x > other_position.x {
                penetration.x = half_total_width - dist_x_abs;
            } else {
                penetration.x = -half_total_width - dist_x_abs;
            }

            Some(Collision { penetration })
        } else {
            None
        }
    }
}
