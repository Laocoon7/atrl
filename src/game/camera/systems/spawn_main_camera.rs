use crate::prelude::*;

use bevy::render::camera::{ScalingMode, WindowOrigin};

pub fn spawn_main_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            left: 0.0,
            right: GRID_WIDTH as f32, //(GRID_WIDTH * MIN_SPRITE_WIDTH) as f32,
            bottom: 0.0,
            top: GRID_HEIGHT as f32, //(GRID_HEIGHT * MIN_SPRITE_HEIGHT) as f32,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::BottomLeft,
            scale: MIN_SPRITE_WIDTH as f32,

            ..Default::default()
        },

        transform: Transform::from_translation(Vec2::ZERO.extend(999.9)),
        ..Default::default()
    };
    commands.spawn((camera, MainCamera));
}

pub fn old_spawn_main_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            left: -0.5,
            right: GRID_WIDTH as f32 - 0.5,
            bottom: -0.5,
            top: GRID_HEIGHT as f32 - 0.5,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::BottomLeft,

            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn((camera, MainCamera));
}
