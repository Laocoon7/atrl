#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

use atrl_engine::{
    bevy::render::texture::ImageSettings,
    bevy_window::{PresentMode, WindowResizeConstraints},
};

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

#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "debug")]
use debug::*;

mod game;
use game::prelude::*;

pub mod raws {
    mod systems {
        mod check_loaded_assets;
        pub use check_loaded_assets::*;
        mod splash;
        pub use splash::*;
    }
    pub use systems::*;

    mod font_assets;
    pub use font_assets::*;
    mod raw_plugin;
    pub use raw_plugin::*;
    pub mod splash_plugin;
    pub use splash_plugin::*;
    mod texture_assets;
    pub use texture_assets::*;
}

mod procgen;

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
    //.insert_resource(bevy::winit::WinitSettings::desktop_app())
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins);

    // anything we don't need in release versions
    #[cfg(feature = "debug")]
    app.add_plugin(DebugPlugin);

    // set entry state
    app.add_loopless_state(GameState::default());

    // asset loading
    app.add_plugin(RawPlugin {
        state_asset_load: GameState::AssetLoad(Load),
        state_asset_load_failure: GameState::AssetLoad(LoadFailure),
    });

    // game related
    app.add_plugin(GamePlugin {
        state_construct: GameState::Construct(MapGen),
        state_main_menu: GameState::Ui(MainMenu),
        state_running: GameState::InGame,
    });

    // `AppState::Initializing` is a buffer state to allow bevy plugins to initialize
    app.add_enter_system(
        GameState::default(),
        switch_in_game_state!(GameState::default().next().unwrap()),
    );

    app.run();
}
