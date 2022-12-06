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

pub mod prelude {
    mod import {
        pub use atrl_camera::prelude::*;
        pub use atrl_data::{
            impl_as_primative, impl_default, impl_new, insert_resource,
            prelude::{AssetLoadState::*, ConstructState::*, UiState::*, *},
            remove_resource, spawn_component, switch_in_game_state,
        };
        pub use atrl_raws::prelude::*;
        pub use atrl_renderer::prelude::*;
        pub use atrl_saveload::prelude::*;
        pub use bevy::{
            ecs::system::{SystemParam, SystemState},
            prelude::*,
            render::once_cell::sync::Lazy, /* From ai::systems::actions::wander, should this not be pub
                                            * use once_cell::Lazy??? */
            utils::{HashMap, HashSet},
        };
        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tileset::prelude::*;
        pub use big_brain::{actions::ActionState as BigBrainActionState, prelude::*};
        pub use index_list::{Index, IndexList};
        pub use iyes_loopless::prelude::*;
        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};
        pub use num_traits::*;
        pub use rand::{distributions::Uniform, prelude::*};
        pub use rand_pcg::Pcg64;
        pub use smart_default::SmartDefault;
    }
    pub(crate) use import::*;

    pub use crate::{ai::*, ecs::*, events::*, game_plugin::*, player::*, spawner::*, turn::*};
}
