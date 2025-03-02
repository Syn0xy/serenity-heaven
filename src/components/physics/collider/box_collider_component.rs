use bevy::math::Vec2;

use super::{CollisionDetection, SphereCollider};

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
        position: &Vec2,
        other_position: &Vec2,
    ) -> bool {
        let half_extents = Vec2::new(self.width / 2.0, self.height / 2.0);
        let min = *other_position - half_extents;
        let max = *other_position + half_extents;

        let closest_point = Vec2::new(
            position.x.clamp(min.x, max.x),
            position.y.clamp(min.y, max.y),
        );

        position.distance(closest_point) <= sphere_collider.radius
    }

    fn collides_with_box(
        &self,
        box_collider: &BoxCollider,
        position: &Vec2,
        other_position: &Vec2,
    ) -> bool {
        (position.x - other_position.x).abs() <= (self.width + box_collider.width) / 2.0
            && (position.y - other_position.y).abs() <= (self.height + box_collider.height) / 2.0
    }
}
