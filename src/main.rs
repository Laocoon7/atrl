#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(unused_imports)] // TODO: REMOVE ME

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod ai {
    mod systems {
        mod scorers {
            mod can_see_player;
            pub use can_see_player::*;
        }
        pub use scorers::*;
        mod actions {
            mod attack;
            pub use attack::*;
            mod chase;
            pub use chase::*;
            mod wander;
            pub use wander::*;
        }
        pub use actions::*;
    }
    pub use systems::*;
    mod ai_plugin;
    pub use ai_plugin::*;
}

mod camera {
    mod resources {
        mod camera_settings_resource;
        pub use camera_settings_resource::*;
        mod loaded_camera;
        pub use loaded_camera::*;
    }
    pub use resources::*;

    mod systems {
        mod spawn_cameras;
        pub use spawn_cameras::*;
    }
    pub use systems::*;

    mod camera_plugin;
    pub use camera_plugin::*;
    mod camera_settings;
    pub use camera_settings::*;
    mod cameras;
    pub use cameras::*;
}

#[cfg(feature = "debug")]
pub(crate) mod debug {
    mod systems {
        mod show_ui;
        mod window_title;
        pub use show_ui::*;
        pub use window_title::*;
    }
    pub use systems::*;

    mod debug_plugin;
    pub use debug_plugin::*;
}

mod ecs {
    mod systems {
        mod apply_damage;
        pub use apply_damage::*;
        mod cull_dead;
        pub use cull_dead::*;
        mod fov;
        pub use fov::*;
        mod perform_healing;
        pub use perform_healing::*;
        mod update_mouse;
        pub use update_mouse::*;
        mod update_targeting;
        pub use update_targeting::*;
    }
    pub use systems::*;

    mod systems_plugin;
    pub use systems_plugin::*;
}

mod events {
    mod map {
        mod map_events;
        pub use map_events::*;
    }
    pub use map::*;

    mod systems {
        mod update_transforms;
        pub use update_transforms::*;
    }
    pub use systems::*;

    mod event_cleaner;
    pub use event_cleaner::*;
    mod event_plugin;
    pub use event_plugin::*;
}

mod player {
    mod systems {
        mod player_input;
        pub use player_input::*;
    }
    pub use systems::*;

    mod player_plugin;
    pub use player_plugin::*;
}

mod spawner {
    mod systems {
        mod spawn_ai;
        pub use spawn_ai::*;
        mod spawn_player;
        pub use spawn_player::*;
    }
    pub use systems::*;

    mod spawner_plugin;
    pub use spawner_plugin::*;
}

mod turn {
    mod actions {
        mod attack;
        pub use attack::*;
        mod movement;
        pub use movement::*;
    }
    pub use actions::*;

    mod systems {
        mod perform_action;
        pub use perform_action::*;
        mod perform_turns;
        pub use perform_turns::*;
    }
    pub use systems::*;

    mod turn_plugin;
    pub use turn_plugin::*;
}

mod game_plugin;
mod log;
pub(crate) mod prelude;
use crate::prelude::*;

fn main() {
    let mut app = App::new();

    // Default Plugins
    default_plugins(&mut app).insert_resource(ClearColor(Color::BLACK));

    // anything we don't need in release versions
    #[cfg(feature = "debug")]
    app.add_plugin(debug::DebugPlugin);

    // game related
    app.add_plugin(GamePlugin {
        state_running: GameState::InGame,
        state_main_menu: GameState::Ui(MainMenu),
        state_asset_load: GameState::AssetLoad(Load),
        state_construct: GameState::Construct(Construct),
        state_construct_setup: GameState::Construct(Setup),
        state_asset_load_failure: GameState::AssetLoad(LoadFailure),
    });

    app.run();
}

fn default_plugins(app: &mut App) -> &mut App {
    let defaults = DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
                title: APP_NAME.to_string(),
                width: (GRID_WIDTH * MIN_SPRITE_WIDTH) as f32 * INITIAL_WINDOW_SCALE,
                height: (GRID_HEIGHT * MIN_SPRITE_HEIGHT) as f32 * INITIAL_WINDOW_SCALE,
                resize_constraints: WindowResizeConstraints {
                    min_width: (GRID_WIDTH * MIN_SPRITE_WIDTH) as f32,
                    min_height: (GRID_HEIGHT * MIN_SPRITE_HEIGHT) as f32,
                    ..Default::default()
                },
                // present_mode: PresentMode::AutoVsync,
                ..Default::default()
            },
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(atlr_log_plugin())
        .build();

    app.add_plugins(defaults);

    #[cfg(feature = "release")]
    {
        defaults.add_before::<bevy::asset::AssetPlugin, _>(bevy_embedded_assets::EmbeddedAssetPlugin);
    }

    app
}
