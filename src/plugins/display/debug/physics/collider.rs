use bevy::prelude::*;

use crate::{
    constants::texture_datas,
    models::{display::Resolution, game::physics::collider::Collider},
};

const COLOR_GREEN: Color = Color::linear_rgb(0.0, 1.0, 0.0);

pub struct DebugColliderPlugin;

impl Plugin for DebugColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_colliders);
    }
}

fn draw_colliders(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Collider)>,
    resolution: Res<Resolution>,
) {
    let scale = resolution.pixel_ratio * texture_datas::TILE_SIZE_F32;
    for (transform, collider) in query.iter() {
        let position = transform.translation.xy();
        match collider {
            Collider::Box(box_collider) => {
                let size = Vec2::new(box_collider.width, box_collider.height) * scale;
                gizmos.rect_2d(position, 0.0, size, COLOR_GREEN);
            }
            Collider::Sphere(sphere_collider) => {
                gizmos.circle_2d(position, sphere_collider.radius * scale, COLOR_GREEN);
            }
        }
    }
}
