#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(clippy::module_inception)]
// The tracking issue for this feature is: [#37854](https://github.com/rust-lang/rust/issues/37854).
// The exclusive_range_pattern feature allows non-inclusive range patterns (0..10)
// to be used in appropriate pattern matching contexts. It also can be combined with
// #![feature(half_open_range_patterns] to be able to use RangeTo patterns (..10).
// !!! It also enabled RangeFrom patterns but that has since been stabilized !!!
#![feature(exclusive_range_pattern)]

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

    pub mod grid_shapes {
        mod circle;
        pub use circle::*;
        mod grid_shape;
        pub use grid_shape::*;
        mod line;
        pub use line::*;
        mod rectangle;
        pub use rectangle::*;
        mod triangle;
        pub use triangle::*;
    }
    pub use grid_shapes::*;

    mod shapes {
        mod iter {
            mod line_iter;
            pub use line_iter::*;
            mod circle_iter;
            pub use circle_iter::*;
            mod rect_iter;
            pub use rect_iter::*;
        }
        pub use iter::*;

        mod ellipse;
        pub use ellipse::*;
        mod polygon;
        pub use polygon::*;
    }
    pub use shapes::*;

    mod shape;
    pub use shape::*;
    mod shape_iter;
    pub use shape_iter::*;
    mod shape_iter_exclusive;
    pub use shape_iter_exclusive::*;
}

mod grid {
    mod point {
        mod point2d;
        pub use point2d::*;
        mod point2d_impl;
        pub use point2d_impl::*;
        mod point2d_iter;
        pub use point2d_iter::*;
    }
    pub use point::*;

    mod grids {
        mod grid_2d;
        pub use grid_2d::*;
        mod bitgrid;
        pub use bitgrid::*;
    }
    pub use grids::*;

    mod axis;
    pub use axis::*;
    mod size2d;
    pub use size2d::*;
    mod grid_layer;
    pub use grid_layer::*;
    mod grid_param;
    pub use grid_param::*;
    mod grid_iterable;
    pub use grid_iterable::*;
}

mod macros {
    mod generic_macros;
    mod switch_in_game_state;
    pub use generic_macros::*;
    pub use switch_in_game_state::*;
    mod primative;
    pub use primative::*;
}

mod random {
    mod noise;
    mod prht;
    mod prng;
    mod random;
    pub use prht::*;
    pub use prng::*;
    pub use random::*;

    pub use self::noise::*;
}

mod states {
    mod state_next;
    pub use state_next::*;
}

mod utils {
    mod file_utils;
    pub use file_utils::*;
    mod range;
    pub use range::*;
    mod white_pixel;
    pub use white_pixel::*;
    mod canvas;
    pub use canvas::*;
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
        pub use bitvec::prelude::*;
        pub use iyes_loopless::prelude::CurrentState;
        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};
        pub use noise::{NoiseFn, Perlin};
        pub use num_derive::*;
        pub use num_traits::*;
        pub use once_cell::sync::Lazy;
        pub use parking_lot::{Mutex, MutexGuard};
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
        pub use crate::{
            common_plugin::*, direction::*, error::*, geometry::*, grid::*, impl_as_primative, impl_default,
            impl_new, insert_resource, interlop::*, macros::*, random::*, remove_resource, spawn_component,
            states::*, switch_in_game_state, utils::*,
        };
    }
    pub use export::*;
}
