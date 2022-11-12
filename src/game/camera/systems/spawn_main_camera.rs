use crate::prelude::*;

use bevy::render::camera::{ScalingMode, WindowOrigin};

pub fn spawn_main_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            left: 0.0,
            right: GRID_WIDTH as f32,
            bottom: 0.0,
            top: GRID_HEIGHT as f32,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::BottomLeft,

            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn_bundle(camera).insert(MainCamera);
}
