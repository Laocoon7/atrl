#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(clippy::module_inception)]

mod actions {
    mod action_type;
    pub use action_type::*;
}

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

    mod blocks {
        mod blocks_movement;
        pub use blocks_movement::*;
        mod blocks_vision;
        pub use blocks_vision::*;
    }
    pub use blocks::*;

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
        pub use local_position::*;
        mod world_position;
        pub use world_position::*;
        mod position;
        pub use position::*;
    }
    pub use position::*;

    mod vision {
        mod vision;
        pub use vision::*;
    }
    pub use vision::*;

    mod consumable;
    pub use consumable::*;
    mod equipable;
    pub use equipable::*;
    mod field_of_view;
    pub use field_of_view::*;
    mod health;
    pub use health::*;
    mod tags;
    pub use tags::*;
    mod target_visualizer;
    pub use target_visualizer::*;
}

mod game {
    mod contexts {
        mod game_context;
        pub use game_context::*;
        mod ai_context;
        pub use ai_context::*;
    }
    pub use contexts::*;

    mod map {
        mod functions {
            mod create_tilemap;
            pub use create_tilemap::*;
        }
        pub use functions::*;

        mod resources {
            mod map_manager;
            pub use map_manager::*;
            mod map_manager_resource;
            pub use map_manager_resource::*;
        }
        pub use resources::*;

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
        mod map_pass_through_data;
        pub use map_pass_through_data::*;
        mod map_plugin;
        pub use map_plugin::*;
    }
    pub use map::*;

    mod game_state;
    pub use game_state::*;
    mod game_stage;
    pub use game_stage::*;
}

pub mod fov {
    mod shadowcast {
        mod shadowcast;
        pub use shadowcast::*;
        mod quadrant;
        mod row;
    }
    pub(crate) use shadowcast::*;

    mod shared {
        mod fov_algorithm;
        pub use fov_algorithm::*;
        mod slope;
        pub use slope::*;
    }
    pub(crate) use shared::*;

    mod fov;
    pub use fov::*;
    mod fov_provider;
    pub use fov_provider::*;
    mod fov_receiver;
    pub use fov_receiver::*;
    mod visibility_map;
    pub use visibility_map::*;
}

mod pathfinding {
    mod astar {
        mod astar;
        pub use astar::*;
        mod astar_node;
    }
    pub use astar::*;

    mod dijkstra {
        // mod dijkstra;
        // pub use dijkstra::*;
    }
    pub use dijkstra::*;

    mod shared {
        mod path_algorithm;
        pub use path_algorithm::*;
    }

    mod pathfinder;
    pub use pathfinder::*;
    mod path_provider;
    pub use path_provider::*;
}

mod resources {
    mod action_queue;
    pub use action_queue::*;
    mod app_settings;
    pub use app_settings::*;
    mod player_entity;
    pub use player_entity::*;
    mod tile_ids;
    pub use tile_ids::*;
    mod tileset_ids;
    pub use tileset_ids::*;
    mod timer;
    pub use timer::*;
    mod font_paths;
    pub use font_paths::*;
    mod turn_manager;
    pub use turn_manager::*;
}

mod queries;
pub mod prelude {
    mod import {
        pub use atrl_common::prelude::*;
        pub use atrl_map_gen::prelude::*;
        pub use bevy::{
            ecs::{
                schedule::StateData,
                system::{SystemParam, SystemState},
            },
            prelude::*,
            render::render_resource::{Extent3d, TextureDimension, TextureFormat},
            utils::{HashMap, HashSet},
        };
        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tileset::prelude::*;
        pub use index_list::{Index, IndexList};
        pub use iyes_loopless::prelude::*;
        pub use leafwing_input_manager::prelude::*;
        pub use num_derive::{FromPrimitive, ToPrimitive};
        pub use num_traits::{FromPrimitive, ToPrimitive};
        pub use once_cell::sync::Lazy;
        pub use ordered_float::OrderedFloat;
        pub use parking_lot::{Mutex, MutexGuard};
        pub use ron;
        pub use serde::{
            de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
            ser::SerializeStruct,
            Deserialize, Serialize,
        };
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::pathfinding::PathFinder; // where's the redundancy lint???
        pub use crate::{
            actions::*, actors::*, camera::*, components::*, fov::*, game::*, pathfinding::*, queries::*,
            resources::*,
        };
    }
    pub use export::*;
}
