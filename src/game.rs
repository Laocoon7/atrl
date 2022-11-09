#![allow(clippy::module_inception)]

mod camera {
    mod systems {
        mod spawn_main_camera;
        pub use spawn_main_camera::*;
    }
    mod camera_plugin;
    pub use camera_plugin::*;
    mod main_camera;
    pub use main_camera::*;
}

mod components {
    mod movement {
        mod movement_type;
        pub use movement_type::*;
        mod movement;
        pub use movement::*;
    }
    pub use movement::*;

    mod position {
        mod local_position;
        pub use local_position::*;
        mod world_position;
        pub use world_position::*;
    }
    pub use position::*;

    mod vision {
        mod vision_type;
        pub use vision_type::*;
        mod vision;
        pub use vision::*;
    }
    pub use vision::*;
}

mod map {
    mod loader {
        mod current_map;
        pub use current_map::*;
        mod map_loader;
        pub use map_loader::*;
    }
    pub use loader::*;

    mod renderer {
        mod map_renderer;
        pub use map_renderer::*;
        mod render_actions;
        pub use render_actions::*;
        mod render_context;
        pub use render_context::*;
        mod tile_bundle;
        pub use tile_bundle::*;
    }
    pub use renderer::*;

    mod systems {
        mod create_renderer;
        pub use create_renderer::*;
        mod draw_map_tiles;
        pub use draw_map_tiles::*;
        mod load_first_map;
        pub use load_first_map::*;
        mod update_map_tiles;
        pub use update_map_tiles::*;
    }

    mod map_layer;
    pub use map_layer::*;
    mod map_plugin;
    pub use map_plugin::*;
    mod map;
    pub use map::*;
}

mod raws {
    pub use crate::raws::*;
}

mod procgen {
    pub use crate::procgen::*;
}

mod game_context;
mod game_plugin;
pub use game_plugin::*; // pub use for main()
mod game_state;

pub mod prelude {
    // Bevy
    pub use atrl_engine::{bevy_utils::HashMap, *};
    pub use atrl_utils::*;

    // Bevy Plugins
    pub use banana_bevy_utils::prelude::*;
    pub use bevy_inspector_egui::prelude::*; // For derive(Inspectable)
    pub use iyes_loopless::prelude::*;

    // Serialization
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;

    // local
    pub use super::camera::*;
    pub use super::components::*;
    pub use super::game_context::*;
    pub use super::map::*;
    pub use super::procgen::*;
    pub use super::raws::*;
    //pub use super::game_plugin::*; // only needed by main()
    pub use super::game_state::*;
}
