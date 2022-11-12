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

mod range;

pub(crate) mod internal {
    pub use atrl_engine::*;
    pub use serde::{Deserialize, Serialize};
}

pub mod prelude {
    pub use super::cp437::*;
    pub use super::direction::*;
    pub use super::geometry::*;
    pub use super::grid::*;
    pub use super::random::*;
    pub use super::range::*;
}
