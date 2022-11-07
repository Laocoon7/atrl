mod camera {
    mod main_camera;
    pub use main_camera::*;
}

mod random {
    mod noise;
    pub use self::noise::*;
    mod prht;
    pub use prht::*;
    mod prng;
    pub use prng::*;
    mod random;
    pub use random::*;
}

mod tilemap {
    mod draw_tilemaps;
    pub use draw_tilemaps::*;
    mod tilemap_action;
    pub use tilemap_action::*;
    mod tilemap_context;
    pub use tilemap_context::*;
    mod tilemap_editor;
    pub use tilemap_editor::*;
    mod tilemap_tile;
    pub use tilemap_tile::*;
    mod tilemap;
    pub use tilemap::*;

    mod tilemap_test_plugin;
    pub use tilemap_test_plugin::*;
}

mod utils {
    mod cp437;
    pub use cp437::*;
    mod range;
    pub use range::*;
}

mod game_assets;
mod game_plugin;
pub use game_plugin::*; // pub use for main()
mod game_state;

pub mod prelude {
    // Bevy
    pub use bevy::prelude::*;

    // Bevy Plugins
    //pub use bevy_ecs_tilemap::prelude::*; // on hold
    pub use bevy_inspector_egui::prelude::*; // For derive(Inspectable)

    // Serialization
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;

    // local
    pub use super::camera::*;
    pub use super::game_assets::*;
    pub use super::random::*;
    pub use super::tilemap::*;
    pub use super::utils::*;
    //pub use super::game_plugin::*; // only needed by main()
    pub use super::game_state::*;
}
