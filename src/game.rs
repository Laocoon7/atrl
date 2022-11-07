#![allow(clippy::module_inception)]

mod camera {
    mod camera_plugin;
    pub use camera_plugin::*;
    mod main_camera;
    pub use main_camera::*;
}

mod map {
    mod map_plugin;
    pub use map_plugin::*;
    mod map_tile_template;
    pub use map_tile_template::*;
    mod map;
    pub use map::*;
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
    mod grid {
        mod grid;
        pub use grid::*;
        mod grid_arithmitic;
        pub use grid_arithmitic::*;
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

mod game_assets;
mod game_plugin;
pub use game_plugin::*; // pub use for main()
mod game_state;

pub mod prelude {
    // Bevy
    pub use bevy::{prelude::*, utils::HashMap};

    // Bevy Plugins
    //pub use bevy_ecs_tilemap::prelude::*; // on hold
    pub use bevy_inspector_egui::prelude::*; // For derive(Inspectable)

    // Serialization
    pub use ron;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;

    // local
    pub use super::camera::*;
    pub use super::game_assets::*;
    pub use super::map::*;
    pub use super::raws::*;
    pub use super::tilemap::*;
    pub use super::utils::*;
    //pub use super::game_plugin::*; // only needed by main()
    pub use super::game_state::*;
}
