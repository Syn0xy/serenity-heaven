use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PlayerController {
    pub direction: IVec2,
}
