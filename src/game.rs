#![allow(clippy::module_inception)]

mod abilities {
    mod ability_plugin;
    pub use ability_plugin::*;
}

mod actors {
    mod actor_bundle;
    pub use actor_bundle::*;
    mod actor_plugin;
    pub use actor_plugin::*;
    mod attributes;
    pub use attributes::*;
    mod class_type;
    pub use class_type::*;
    mod equipment_slots;
    pub use equipment_slots::*;
    mod race_type;
    pub use race_type::*;
    mod timing;
    pub use timing::*;
}

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
    mod ai_component {
        mod ai_component;
        pub use ai_component::*;
        mod ai_type;
        pub use ai_type::*;
    }
    pub use ai_component::*;
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

    mod consumable;
    pub use consumable::*;
    mod equipable;
    pub use equipable::*;
    mod health;
    pub use health::*;
    mod tags;
    pub use tags::*;
}

mod items {
    mod item_bundle;
    pub use item_bundle::*;
    mod item_plugin;
    pub use item_plugin::*;
}

mod map {
    mod loader {
        mod change_theme;
        pub use change_theme::*;
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
        mod change_map_theme;
        pub use change_map_theme::*;
        mod create_renderer;
        pub use create_renderer::*;
        mod draw_map_tiles;
        pub use draw_map_tiles::*;
        mod load_first_map;
        pub use load_first_map::*;
        mod update_map_tiles;
        pub use update_map_tiles::*;
    }

    mod tiles {
        mod loader {
            mod color_definition;
            pub use color_definition::*;
            mod tile_template;
            pub use tile_template::*;
            mod texture_atlas_template;
            pub use texture_atlas_template::*;
            mod tile_loader;
            pub use tile_loader::*;
        }
        pub use loader::*;
        mod theme;
        pub use theme::*;
        mod tile_definition;
        pub use tile_definition::*;
    }
    pub use tiles::*;

    mod feature_type;
    pub use feature_type::*;
    mod item_type;
    pub use item_type::*;
    mod map_layer;
    pub use map_layer::*;
    mod map_plugin;
    pub use map_plugin::*;
    mod map;
    pub use map::*;
    mod terrain_type;
    pub use terrain_type::*;
}

mod systems {
    mod apply_damage;
    pub use apply_damage::*;
    mod input;
    pub use input::*;
    mod move_actors;
    pub use move_actors::*;
    mod perform_healing;
    pub use perform_healing::*;
}

mod player {
    mod systems {
        mod player_input;
        pub use player_input::*;
    }
    pub use systems::*;

    mod player_bundle;
    pub use player_bundle::*;
    mod player_plugin;
    pub use player_plugin::*;
}

mod spawner {
    mod systems {
        mod spawn_player;
        pub use spawn_player::*;
    }
    pub use systems::*;

    mod spawner_plugin;
    pub use spawner_plugin::*;
}

mod game_context;
mod game_plugin;
mod game_state;

pub mod prelude {
    // Bevy
    pub use atrl_engine::{bevy_utils::HashMap, *};
    pub use atrl_utils::prelude::*;

    // Bevy Plugins
    pub use bevy_inspector_egui::prelude::*; // For derive(Inspectable)
    pub use leafwing_input_manager::prelude::*; // Input
    pub use leafwing_input_manager::Actionlike; // Input

    pub use iyes_loopless;
    pub use iyes_loopless::prelude::*;
    pub use iyes_progress;
    pub use iyes_progress::prelude::*;

    // Serialization
    pub use ron;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;

    // Our external Crates
    pub use crate::procgen::*;
    pub use crate::raws::*;

    // local
    pub use super::abilities::*;
    pub use super::actors::*;
    pub use super::camera::*;
    pub use super::components::*;
    pub use super::items::*;
    pub use super::map::*;
    pub use super::player::*;
    pub use super::spawner::*;

    pub use super::game_context::*;
    pub use super::game_plugin::*;
    pub use super::game_state::AssetLoadStates::*;
    pub use super::game_state::ConstructStates::*;
    pub use super::game_state::UiStates::*;
    pub use super::game_state::*;
}
