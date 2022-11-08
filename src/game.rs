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

    mod vision {
        mod vision_type;
        pub use vision_type::*;
        mod vision;
        pub use vision::*;
    }
    pub use vision::*;
}

mod map {
    mod create_tilemaps;
    pub use create_tilemaps::*;
    mod current_map;
    pub use current_map::*;
    mod map_plugin;
    pub use map_plugin::*;
    mod map_tile_template;
    pub use map_tile_template::*;
    mod map;
    pub use map::*;
    mod tilemap_id;
    pub use tilemap_id::*;
    mod update_map_tiles;
    pub use update_map_tiles::*;
}

mod map_new {
    mod loader {
        mod systems {}
        pub use systems::*;

        mod current_map;
        pub use current_map::*;
        mod map_loader_plugin;
        pub use map_loader_plugin::*;
        mod map_loader;
        pub use map_loader::*;
    }
    pub use loader::*;

    mod renderer {
        mod systems {}
        pub use systems::*;

        mod map_renderer_plugin;
        pub use map_renderer_plugin::*;
        mod map_renderer;
        pub use map_renderer::*;
    }
    pub use renderer::*;

    mod systems {}
    pub use systems::*;

    mod map_plugin;
    pub use map_plugin::*;
    mod map;
    pub use map::*;
    mod tile_definition;
    pub use tile_definition::*;
}

mod tilemap {
    mod draw_tilemaps;
    pub use draw_tilemaps::*;
    mod tile_definition;
    pub use tile_definition::*;
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
    mod grid {
        mod grid;
        pub use grid::*;
        mod grid_arithmetic;
        pub use grid_arithmetic::*;
    }
    pub use grid::*;

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
    pub use random::*;

    mod shapes {
        mod circle;
        pub use circle::*;
        mod line;
        pub use line::*;
        mod polygon;
        pub use polygon::*;
        mod ray;
        pub use ray::*;
        mod rectangle;
        pub use rectangle::*;
        mod segment;
        pub use segment::*;
        mod triangle;
        pub use triangle::*;
    }
    pub use shapes::*;
}

mod raws {
    pub use crate::raws::*;
}

mod game_plugin;
pub use game_plugin::*; // pub use for main()
mod game_state;

pub mod prelude {
    // Bevy
    pub use bevy::{prelude::*, utils::HashMap};

    // Bevy Plugins
    pub use banana_bevy_utils::prelude::*;
    pub use bevy_inspector_egui::prelude::*; // For derive(Inspectable)
    pub use iyes_loopless::prelude::*;

    // Serialization
    pub use ron;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;

    // local
    pub use super::camera::*;
    pub use super::components::*;
    pub use super::map::*;
    pub use super::raws::*;
    pub use super::tilemap::*;
    pub use super::utils::*;
    //pub use super::game_plugin::*; // only needed by main()
    pub use super::game_state::*;
}
