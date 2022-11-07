use banana_bevy_utils::switch_in_game_state;
use iyes_loopless::prelude::AppLooplessStateExt;

pub mod app_settings {
    /// Long name
    pub const APP_NAME: &str = "Away Team Roguelike";
    /// Short name
    pub const _APP_NAME_SHORT: &str = "ATRL";
    /// Grid width in cells
    pub const GRID_WIDTH: u32 = 80;
    /// Grid height in cells
    pub const GRID_HEIGHT: u32 = 45;
    /// Minimum sprite width expected
    pub const MIN_SPRITE_WIDTH: u32 = 8;
    /// Minimum sprite height expected
    pub const MIN_SPRITE_HEIGHT: u32 = 8;
    /// Startup window size will be scaled by this value
    pub const INITIAL_WINDOW_SCALE: f32 = 2.0;
}
use app_settings::*;

mod debug;
use debug::*;

mod game;
use game::{prelude::GameState, *};

mod raws {
    mod raw_plugin;
    pub use raw_plugin::*;
}
use raws::*;

use bevy::{
    prelude::*,
    render::texture::ImageSettings,
    window::{PresentMode, WindowResizeConstraints},
    winit::WinitSettings,
};

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
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
    // bevy-inspector-egui related
    .add_plugin(DebugPlugin)
    // Tilemap (on hold)
    //.add_plugin(TilemapPlugin)
    // game related
    .add_plugin(GamePlugin {
        state_asset_load: GameState::AssetLoad,
        state_construct: GameState::Construct,
        state_main_menu: GameState::MainMenu,
        state_running: GameState::InGame,
    })
    .add_plugin(RawPlugin {
        state_asset_load: GameState::AssetLoad,
        state_construct: GameState::Construct,
    });

    // `AppState::Initializing` is a buffer state to allow bevy plugins to initialize
    app.add_enter_system(GameState::default(), switch_in_game_state!(GameState::AssetLoad));

    app.run();
}
