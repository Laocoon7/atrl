mod builders {
    mod cellular_automata;
    pub use cellular_automata::*;
    mod rooms;
    pub use rooms::*;
}
pub use builders::*;

mod meta {
    mod area_points {
        mod area_ending_point;
        pub use area_ending_point::*;
        mod area_starting_points;
        pub use area_starting_points::*;
    }
    pub use area_points::*;
    mod cull_unreachable;
    pub use cull_unreachable::*;
}
pub use meta::*;

mod builder_chain;
pub use builder_chain::*;
mod common;
pub use common::*;
mod map_builder;
pub use map_builder::*;

pub mod prelude {
    // Files inside of atrl::procgen *may*
    // use crate::procgen::prelude::internal::*;
    // Files outside of atrl::procgen should only
    // access procgen from crate::prelude::*;
    pub mod internal {
        pub use super::super::builder_chain::*;
        pub use super::super::builders::*;
        pub use super::super::common::*;
        pub use super::super::map_builder::*;
        pub use super::super::meta::*;
    }

    // No pub use here, explicit folder for procgen internal data
    pub mod external {
        pub use super::super::builder_chain::*;
    }
}
