use banana_bevy_utils::switch_in_game_state;

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

mod procgen {
    mod builder_chain;
    mod builders;
    mod common;
    mod map_builder;
    mod procgen_plugin;

    mod meta {
        mod area_points;
        mod cull_unreachable;
        pub use area_points::*;
        pub use cull_unreachable::*;
    }

    pub use builder_chain::*;
    pub use builders::*;
    pub use common::*;
    pub use map_builder::*;
    pub use meta::*;
    pub use procgen_plugin::*;
}
use procgen::*;

use atrl_engine::{
    bevy::render::texture::ImageSettings,
    bevy_window::{PresentMode, WindowResizeConstraints},
    *,
};

fn main() {
    let mut app = App::new();
    // Default Plugins
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
    //.insert_resource(WinitSettings::desktop_app())
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins);

    // anything we don't need in release versions
    app.add_plugin(DebugPlugin);

    // set entry state
    app.add_loopless_state(GameState::default());

    // asset loading
    app.add_plugin(RawPlugin {
        state_asset_load: GameState::AssetLoad,
        state_construct: GameState::Construct,
    });

    // game related
    app.add_plugin(GamePlugin {
        state_asset_load: GameState::AssetLoad,
        state_construct: GameState::Construct,
        state_main_menu: GameState::MainMenu,
        state_running: GameState::InGame,
    });

    // `AppState::Initializing` is a buffer state to allow bevy plugins to initialize
    app.add_enter_system(GameState::default(), switch_in_game_state!(GameState::AssetLoad));

    app.run();
}
