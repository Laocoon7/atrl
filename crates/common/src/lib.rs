#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(clippy::module_inception)]

mod direction {
    mod bitmap;
    mod cardinal;
    mod direction;
    mod iter;
    mod ordinal;
    mod table;
    pub use bitmap::*;
    pub use cardinal::*;
    pub use direction::*;
    pub use iter::*;
    pub use ordinal::*;
    pub use table::*;
}

mod geometry {
    mod math {
        mod distance;
        mod intersects;
        mod lerp;
        mod rotate_points;
        mod scale_points;
        pub use distance::*;
        pub use intersects::*;
        pub use lerp::*;
        pub use rotate_points::*;
        pub use scale_points::*;
    }
    pub use math::*;

    mod shapes {
        mod circle;
        mod ellipse;
        mod line;
        mod polygon;
        mod rectangle;
        mod triangle;
        pub use circle::*;
        pub use ellipse::*;
        pub use line::*;
        pub use polygon::*;
        pub use rectangle::*;
        pub use triangle::*;
    }
    pub use shapes::*;

    mod shape;
    pub use shape::*;
}

mod macros {
    mod generic_macros;
    mod switch_in_game_state;
    pub use generic_macros::*;
    pub use switch_in_game_state::*;
}

mod random {
    mod noise;
    mod prht;
    mod prng;
    mod random;
    pub use self::noise::*;
    pub use prht::*;
    pub use prng::*;
    pub use random::*;
}

mod states {
    mod state_next;
    pub use state_next::*;
}

mod utils {
    mod fov {
        mod visibility_flags;
        pub use visibility_flags::*;
        mod visibility_map_2d;
        pub use visibility_map_2d::*;
        mod visibility_map;
        pub use visibility_map::*;
    }
    pub use fov::*;

    mod axis;
    mod bitgrid;
    mod file_utils;
    mod grid;
    mod grid_packer;
    mod point2d;
    mod range;
    mod size2d;
    mod white_pixel;
    pub use axis::*;
    pub use bitgrid::*;
    pub use file_utils::*;
    pub use grid::*;
    pub use grid_packer::*;
    pub use point2d::*;
    pub use range::*;
    pub use size2d::*;
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

        pub use bevy_ecs_tilemap::prelude::TilePos;
        pub use iyes_loopless::prelude::CurrentState;
        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};

        pub use num_derive::*;
        pub use num_traits::*;

        pub use bitvec::prelude::*;
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
        pub use crate::geometry::*;

        pub use crate::macros::*;
        pub use crate::switch_in_game_state;
        pub use crate::{
            impl_default, impl_new, insert_resource, remove_resource, spawn_component,
        };

        pub use crate::random::*;
        pub use crate::states::*;
        pub use crate::utils::*;

        pub use crate::common_plugin::*;
        pub use crate::error::*;
        pub use crate::interlop::*;
    }
    pub use export::*;
}
