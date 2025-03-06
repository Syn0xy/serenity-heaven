use bevy::math::Vec2;

use super::{collides_box_and_sphere, BoxCollider, Collision, CollisionDetection};

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
        position: Vec2,
        other_position: Vec2,
    ) -> Option<Collision> {
        let total_radius = self.radius + sphere_collider.radius;
        let dist = position.distance(other_position);
        let delta = total_radius - dist;

        if delta >= 0.0 {
            let dir = (position - other_position).normalize();

            Some(Collision {
                penetration: dir * delta,
            })
        } else {
            None
        }
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
