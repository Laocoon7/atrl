#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(clippy::module_inception)]

mod actors {
    mod class_type;
    pub use class_type::*;
    mod equipment_slot;
    pub use equipment_slot::*;
    mod movement_type;
    pub use movement_type::*;
    mod player_action;
    pub use player_action::*;
    mod race_type;
    pub use race_type::*;
    mod vision_type;
    pub use vision_type::*;
}
pub use actors::*;

mod camera {
    mod camera_id;
    pub use camera_id::*;
}
pub use camera::*;

mod components {
    pub use bevy::{
        prelude::{Component, ReflectComponent},
        reflect::Reflect,
    };

    mod ai_component {
        mod ai_component;
        mod ai_type;
        pub use ai_component::*;
        pub use ai_type::*;
    }
    pub use ai_component::*;

    mod bundles {
        mod actor_bundle;
        mod player_bundle;
        pub use actor_bundle::*;
        pub use player_bundle::*;
    }
    pub use bundles::*;

    mod movement {
        mod movement;
        pub use movement::*;
    }
    pub use movement::*;

    mod position {
        mod local_position;
        mod world_position;
        pub use local_position::*;
        pub use world_position::*;
    }
    pub use position::*;

    mod vision {
        mod vision;
        pub use vision::*;
    }
    pub use vision::*;

    mod consumable;
    mod equipable;
    mod field_of_view;
    mod health;
    mod tags;
    pub use consumable::*;
    pub use equipable::*;
    pub use field_of_view::*;
    pub use health::*;
    pub use tags::*;
}

mod game {
    mod map {
        mod tiles {
            mod feature_type;
            pub use feature_type::*;
            mod item_type;
            pub use item_type::*;
            mod terrain_type;
            pub use terrain_type::*;
        }
        pub use tiles::*;

        mod map;
        pub use map::*;
        mod map_layer;
        pub use map_layer::*;
    }
    pub use map::*;

    mod game_context;
    pub use game_context::*;
    mod game_state;
    pub use game_state::*;
}

// Leave this as pub so that we can do `fov::compute` and not `compute`
pub mod fov {
    mod adams_fov;
    mod slope;
    mod visibility_compute;
    mod visibility_provider;
    pub use adams_fov::*;
    pub use slope::*;
    pub use visibility_compute::*;
    pub use visibility_provider::*;
}

mod system_params {
    mod player_param;
    pub use player_param::*;
}

mod app_settings;

pub mod prelude {

    mod import {
        pub use bevy::{
            ecs::{
                schedule::StateData,
                system::{SystemParam, SystemState},
            },
            prelude::*,
            render::render_resource::{Extent3d, TextureDimension, TextureFormat},
            utils::HashSet,
        };

        pub use iyes_loopless::prelude::*;

        pub use leafwing_input_manager::prelude::*;

        pub use num_derive::{FromPrimitive, ToPrimitive};
        pub use num_traits::{FromPrimitive, ToPrimitive};

        pub use once_cell::sync::Lazy;

        pub use parking_lot::{Mutex, MutexGuard};

        pub use ron;
        pub use serde::{
            de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
            ser::SerializeStruct,
            Deserialize, Serialize,
        };

        pub use atrl_common::prelude::*;
        pub use atrl_map_gen::prelude::*;
    }
    pub(crate) use import::*;

    mod internal {
        pub use crate::fov::*;
    }
    pub(crate) use internal::*;

    mod export {
        pub use crate::actors::*;
        pub use crate::camera::*;
        pub use crate::components::*;
        pub use crate::game::*;
        pub use crate::system_params::*;

        pub use crate::fov;

        pub use crate::app_settings::*;
    }
    pub use export::*;
}
