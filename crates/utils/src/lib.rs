#![allow(clippy::module_inception)]

mod cp437;
pub use cp437::*;

mod range;
pub use range::*;

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
pub use direction::*;

mod grid {
    mod axis;
    mod grid;
    mod grid_arithmetic;

    pub use axis::*;
    pub use grid::*;
    pub use grid_arithmetic::*;
}
pub use grid::*;

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
pub use random::*;

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
pub use geometry::*;

pub(crate) mod prelude {
    pub use atrl_engine::*;
    pub use serde::{Deserialize, Serialize};
}
