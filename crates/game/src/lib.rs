#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]

mod game_map {
    mod resources {
        mod map_manager;
        pub use map_manager::*;
    }
    pub use resources::*;

    mod systems {
        mod load_first_map;
        pub use load_first_map::*;
        mod update_tilemaps;
        pub use update_tilemaps::*;
        mod wait_for_tilesets_to_load;
        pub use wait_for_tilesets_to_load::*;
    }
    pub use systems::*;

    mod game_map_plugin;
    pub use game_map_plugin::*;
}

mod map_gen {
    mod builders {
        mod cellular_automata;
        pub use cellular_automata::*;
        mod rooms;
        pub use rooms::*;
    }
    pub use builders::*;

    mod meta {
        mod area_points {
            mod area_ending_point;
            pub use area_ending_point::*;
            mod area_starting_point;
            pub use area_starting_point::*;
        }
        pub use area_points::*;

        mod cull_unreachable;
        pub use cull_unreachable::*;
    }
    pub use meta::*;

    mod builder_chain;
    pub use builder_chain::*;
    mod common;
    pub use common::*;
    mod map_builder;
    pub use map_builder::*;
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

mod spawner {
    mod systems {
        mod spawn_player;
        pub use spawn_player::*;
    }
    pub use systems::*;

    mod spawner_plugin;
    pub use spawner_plugin::*;
}

mod game_plugin;
mod turn_state_plugin;

pub mod prelude {
    mod import {
        pub use bevy::{
            prelude::*,
            utils::{HashMap, HashSet},
        };

        pub use banana_bevy_utils::prelude::*;

        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tileset::prelude::*;

        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};

        pub use iyes_loopless::prelude::*;

        pub use smart_default::SmartDefault;

        pub use atrl_camera::prelude::*;
        pub use atrl_common::prelude::AssetLoadState::*;
        pub use atrl_common::prelude::ConstructState::*;
        pub use atrl_common::prelude::TurnState::*;
        pub use atrl_common::prelude::UiState::*;
        pub use atrl_common::prelude::*;
        pub use atrl_map::prelude::*;
        pub use atrl_raws::prelude::*;
        pub use atrl_saveload::prelude::*;
        pub use atrl_ui::prelude::*;
    }
    pub(crate) use import::*;

    pub use crate::game_map::*;
    pub use crate::map_gen::*;
    pub use crate::player::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    pub use crate::game_plugin::*;
    pub use crate::turn_state_plugin::*;
}

//pub use crate::procgen::*;
//pub use crate::map::*;
