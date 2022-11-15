#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

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
    mod health;
    mod tags;
    pub use consumable::*;
    pub use equipable::*;
    pub use health::*;
    pub use tags::*;
}

mod data {
    mod actors {
        mod class_type;
        pub use class_type::*;
        mod equipment_slot;
        pub use equipment_slot::*;
        mod player_action;
        pub use player_action::*;
        mod race_type;
        pub use race_type::*;
    }
    pub use actors::*;

    mod camera {
        mod camera_id;
        pub use camera_id::*;
    }
    pub use camera::*;

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

        mod map_layer;
        pub use map_layer::*;
        mod movement_type;
        pub use movement_type::*;
        mod vision_type;
        pub use vision_type::*;
    }
    pub use map::*;

    mod game_context;
    pub use game_context::*;
    mod game_state;
    pub use game_state::*;
}

mod direction {
    mod bitmap;
    pub use bitmap::*;
    mod cardinal;
    pub use cardinal::*;
    mod direction;
    pub use direction::*;
    mod iter;
    pub use iter::*;
    mod ordinal;
    pub use ordinal::*;
    mod table;
    pub use table::*;
}

mod grid {
    mod axis;
    pub use axis::*;
    mod grid;
    pub use grid::*;
    mod point2d;
    pub use point2d::*;
    mod size2d;
    pub use size2d::*;
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

mod geometry {
    mod distance;
    pub use distance::*;

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

mod app_settings;
mod common_plugin;
mod cp437;
mod error;
mod file_utils;
mod interlop;
mod range;
mod white_pixel;

pub mod prelude {
    mod import {
        pub use banana_bevy_utils::prelude::*;
        pub use bevy::{
            ecs::system::SystemState,
            prelude::*,
            render::render_resource::{Extent3d, TextureDimension, TextureFormat},
            utils::HashSet,
        };
        pub use iyes_loopless::prelude::CurrentState;

        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};

        pub use num_derive::*;
        pub use num_traits::*;

        pub use once_cell::sync::Lazy;

        pub use parking_lot::{Mutex, MutexGuard};

        pub use noise::{NoiseFn, Perlin};
        pub use rand::{distributions::Standard, prelude::*};
        pub use rand_pcg::Pcg64;
        pub use serde::{
            de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
            ser::SerializeStruct,
            Deserialize, Serialize,
        };
        pub use thiserror::Error;
        pub use xxhash_rust::xxh3::*;
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::components::*;
        pub use crate::data::*;
        pub use crate::direction::*;
        pub use crate::geometry::*;
        pub use crate::grid::*;
        pub use crate::random::*;

        pub use crate::app_settings::*;
        pub use crate::common_plugin::*;
        pub use crate::cp437::*;
        pub use crate::error::*;
        pub use crate::file_utils::*;
        pub use crate::interlop::*;
        pub use crate::range::*;
        pub use crate::white_pixel::*;
    }
    pub use export::*;
}
