use bevy::math::Vec2;

use super::{BoxCollider, CollisionDetection};

#[derive(Default, Debug)]
pub struct SphereCollider {
    pub radius: f32,
}

impl SphereCollider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl CollisionDetection for SphereCollider {
    fn collides_with_sphere(
        &self,
        sphere_collider: &SphereCollider,
        position: &Vec2,
        other_position: &Vec2,
    ) -> bool {
        position.distance(*other_position) <= self.radius + sphere_collider.radius
    }

    fn collides_with_box(
        &self,
        box_collider: &BoxCollider,
        position: &Vec2,
        other_position: &Vec2,
    ) -> bool {
        let half_extents = Vec2::new(box_collider.width / 2.0, box_collider.height / 2.0);
        let min = *other_position - half_extents;
        let max = *other_position + half_extents;

        let closest_point = Vec2::new(
            position.x.clamp(min.x, max.x),
            position.y.clamp(min.y, max.y),
        );

        position.distance(closest_point) <= self.radius
    }
}
