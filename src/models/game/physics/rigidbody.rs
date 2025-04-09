use bevy::prelude::*;

use crate::constants::rigidbody_datas;

#[derive(Component, Debug)]
pub struct Rigidbody {
    pub mass: f32,
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
        Self::new(rigidbody_datas::DEFAULT_MASS)
    }
}

impl Rigidbody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
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

    pub fn update(&mut self) {
        self.velocity = (self.force / self.mass) + self.acceleration;
        self.current_speed = self.velocity.length();
        self.force = Vec2::ZERO;
    }
}
