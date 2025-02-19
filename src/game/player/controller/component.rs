use bevy::{ecs::component::Component, math::IVec2};

#[derive(Default, Component)]
pub struct PlayerController {
    pub direction: IVec2,
}
