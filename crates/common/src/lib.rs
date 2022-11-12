#![allow(clippy::module_inception)]

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
    mod point_iter;
    pub use point_iter::*;
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

mod cp437;
mod game_state;
mod interlop;
mod range;

pub mod prelude {
    mod import {
        pub use atrl_components::prelude::*;
        pub use banana_bevy_utils::prelude::*;
        pub use bevy::{prelude::*, utils::HashSet};
        pub use iyes_loopless::prelude::CurrentState;

        pub use noise::{NoiseFn, Perlin};
        pub use rand::{distributions::Standard, prelude::*};
        pub use rand_pcg::Pcg64;
        pub use serde::{
            de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
            ser::SerializeStruct,
            Deserialize, Serialize,
        };
        pub use xxhash_rust::xxh3::*;
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::cp437::*;
        pub use crate::direction::*;
        pub use crate::game_state::*;
        pub use crate::geometry::*;
        pub use crate::grid::*;
        pub use crate::interlop::*;
        pub use crate::random::*;
        pub use crate::range::*;
    }
    pub use export::*;
}
