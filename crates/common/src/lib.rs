#![allow(clippy::module_inception)]
#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

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

mod events {
    mod event_plugin;
    pub use event_plugin::*;
}

pub mod fov {
    mod adams_fov;
    mod visibility_compute;
    mod visibility_map;

    pub use adams_fov::*;
    pub use visibility_compute::*;
    pub use visibility_map::*;
}

mod geometry {
    mod math {
        mod distance;
        pub use distance::*;
        mod intersects;
        pub use intersects::*;
        mod lerp;
        pub use lerp::*;
        mod rotate_points;
        pub use rotate_points::*;
        mod scale_points;
        pub use scale_points::*;
    }
    pub use math::*;

    mod shapes {
        mod circle;
        pub use circle::*;
        mod ellipse;
        pub use ellipse::*;
        mod line;
        pub use line::*;
        mod polygon;
        pub use polygon::*;
        mod rectangle;
        pub use rectangle::*;
        mod triangle;
        pub use triangle::*;
    }
    pub use shapes::*;

    mod shape;
    pub use shape::*;
}

mod macros {
    mod generic_macros;
    pub use generic_macros::*;
    mod switch_in_game_state;
    pub use switch_in_game_state::*;
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

mod states {
    mod state_next;
    pub use state_next::*;
}

mod utils {
    mod axis;
    pub use axis::*;
    mod file_utils;
    pub use file_utils::*;
    mod grid;
    pub use grid::*;
    mod point2d;
    pub use point2d::*;
    mod range;
    pub use range::*;
    mod size2d;
    pub use size2d::*;
    mod white_pixel;
    pub use white_pixel::*;
}

mod common_plugin;
mod error;
mod interlop;

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

        pub use bitvec::prelude::*;

        pub use iyes_loopless::prelude::CurrentState;

        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};

        pub use num_derive::*;
        pub use num_traits::*;

        pub use once_cell::sync::Lazy;

        pub use parking_lot::{Mutex, MutexGuard};

        pub use noise::{NoiseFn, Perlin};
        pub use rand::{distributions::Standard, prelude::*};
        pub use rand_pcg::Pcg64;
        pub use ron;
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
        pub use crate::direction::*;
        pub use crate::events::*;
        pub use crate::fov::*;
        pub use crate::geometry::*;

        pub use crate::macros::*;
        pub use crate::switch_in_game_state;
        pub use crate::{
            impl_default, impl_new, insert_resource, remove_resource, spawn_component,
        };

        pub use crate::random::*;
        pub use crate::states::*;
        pub use crate::utils::*;

        pub use crate::fov;

        pub use crate::common_plugin::*;
        pub use crate::error::*;
        pub use crate::interlop::*;
    }
    pub use export::*;
}
