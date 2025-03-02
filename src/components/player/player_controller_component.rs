use bevy::{ecs::component::Component, math::IVec2};

#[derive(Component, Default)]
pub struct PlayerController {
    pub direction: IVec2,
}
