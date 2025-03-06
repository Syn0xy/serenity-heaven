use bevy::prelude::*;

use crate::constants::rigidbody_datas;

#[derive(Component, Debug)]
pub struct Rigidbody {
    pub mass: f32,
    pub drag: f32,
    pub force: Vec2,
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub current_speed: f32,
}

#[derive(Debug)]
pub enum ForceMode {
    Force,
    Acceleration,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self::new(rigidbody_datas::DEFAULT_MASS, rigidbody_datas::DEFAULT_DRAG)
    }
}

impl Rigidbody {
    pub fn new(mass: f32, drag: f32) -> Self {
        Self {
            mass,
            drag,
            force: Vec2::default(),
            acceleration: Vec2::default(),
            velocity: Vec2::default(),
            current_speed: f32::default(),
        }
    }

    pub fn add_force(&mut self, new_force: Vec2, force_mode: ForceMode) {
        match force_mode {
            ForceMode::Force => self.force += new_force,
            ForceMode::Acceleration => self.acceleration = new_force,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let previous_velocity = self.velocity.clone();
        let friction_factor = (-self.drag * delta_time).exp();
        let add_vel = (self.force / self.mass) + self.acceleration;

        self.velocity = add_vel * friction_factor;
        self.current_speed = previous_velocity.distance_squared(self.velocity);
        self.force = Vec2::ZERO;
    }
}
