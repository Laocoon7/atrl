pub mod app_settings {
    pub const APP_NAME: &str = "Away Team Roguelike";
    pub const _APP_NAME_SHORT: &str = "ATRL";
    pub const GRID_WIDTH: u32 = 80;
    pub const GRID_HEIGHT: u32 = 45;
    pub const MIN_SPRITE_WIDTH: u32 = 8;
    pub const MIN_SPRITE_HEIGHT: u32 = 8;
    pub const INITIAL_WINDOW_SCALE: f32 = 2.0;
}
use app_settings::*;

mod debug;
use debug::*;

mod game;
use game::*;

use bevy::{
    prelude::*,
    render::texture::ImageSettings,
    window::{PresentMode, WindowResizeConstraints},
    winit::WinitSettings,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: APP_NAME.to_string(),
            width: (GRID_WIDTH * MIN_SPRITE_WIDTH) as f32 * INITIAL_WINDOW_SCALE,
            height: (GRID_HEIGHT * MIN_SPRITE_HEIGHT) as f32 * INITIAL_WINDOW_SCALE,
            resize_constraints: WindowResizeConstraints {
                min_width: (GRID_WIDTH * MIN_SPRITE_WIDTH) as f32,
                min_height: (GRID_HEIGHT * MIN_SPRITE_HEIGHT) as f32,
                ..Default::default()
            },
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(GamePlugin)
        .run();
}
