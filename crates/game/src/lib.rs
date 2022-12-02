#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(unused_imports)] // TODO: REMOVE ME
mod ai {
    mod systems {
        mod scorers {
            mod can_see_player;
            pub use can_see_player::*;
        }
        pub use scorers::*;
        mod actions {
            mod chase;
            mod wander;
            pub use chase::*;
            pub use wander::*;
        }
        pub use actions::*;
    }
    pub use systems::*;
    mod ai_plugin;
    pub use ai_plugin::*;
}

mod events {
    mod map {
        mod map_events;
        pub use map_events::*;
    }
    pub use map::*;

    mod event_cleaner;
    pub use event_cleaner::*;
}

mod map {
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
    }
    pub use systems::*;

    mod map_plugin;
    pub use map_plugin::*;
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
        mod spawn_player;
        pub use spawn_ai::*;
        pub use spawn_player::*;
    }
    pub use systems::*;

    mod spawner_plugin;
    pub use spawner_plugin::*;
}

mod systems {
    mod action_queue;
    pub use action_queue::*;
    mod apply_damage;
    pub use apply_damage::*;
    mod fov;
    pub use fov::*;
    mod perform_healing;
    pub use perform_healing::*;
    mod perform_turns;
    pub use perform_turns::*;
    mod movement;
    pub use movement::*;
    mod cull_dead;
    pub use cull_dead::*;
}

mod ecs_plugin;
mod game_plugin;

pub mod prelude {
    mod import {
        pub use atrl_camera::prelude::*;
        pub use atrl_common::prelude::*;
        pub use atrl_data::prelude::{AssetLoadState::*, ConstructState::*, TurnState::*, UiState::*, *};
        pub use atrl_map_gen::prelude::*;
        pub use atrl_raws::prelude::*;
        pub use atrl_renderer::prelude::*;
        pub use atrl_saveload::prelude::*;
        pub use bevy::{
            prelude::*,
            utils::{HashMap, HashSet},
        };
        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tileset::prelude::*;
        pub use big_brain::{actions::ActionState as BigBrainActionState, prelude::*};
        pub use index_list::{Index, IndexList};
        pub use iyes_loopless::prelude::*;
        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};
        pub use num_traits::{FromPrimitive, ToPrimitive};
        pub use rand::prelude::*;
        pub use rand_pcg::Pcg64;
        pub use smart_default::SmartDefault;
    }
    pub(crate) use import::*;

    pub use crate::{
        ai::*, ecs_plugin::*, events::*, game_plugin::*, map::*, player::*, spawner::*, systems::*,
    };
}
