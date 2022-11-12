#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

pub mod app_settings;
pub mod game;
pub mod procgen;

pub(crate) mod prelude;
use crate::prelude::*;

#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "debug")]
use debug::*;

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
