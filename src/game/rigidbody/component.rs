use bevy::{ecs::component::Component, math::Vec2};

use crate::game::GTransform;

const DEFAULT_MASS: f32 = 1.0;

#[derive(Component)]
pub struct Rigidbody {
    pub mass: f32,
    pub force: Vec2,
    pub velocity: Vec2,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self::new(DEFAULT_MASS)
    }
}

impl Rigidbody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            force: Vec2::ZERO,
            velocity: Vec2::ZERO,
        }
    }

    pub fn apply_force(&mut self, new_force: Vec2) {
        self.force += new_force
    }

    pub fn update(&mut self, gtransform: &mut GTransform, delta_time: f32) {
        let acceleration = self.force / self.mass;

        self.force = Vec2::ZERO;
        self.velocity += acceleration * delta_time;
        gtransform.position += self.velocity * delta_time;
    }
}
