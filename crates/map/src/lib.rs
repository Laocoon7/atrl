#![allow(clippy::type_complexity)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

mod functions {
    mod create_tilemap;
    pub use create_tilemap::*;
}

mod procgen {
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
}

mod systems {
    mod load_tilesets;
    pub use load_tilesets::*;

    mod gen_tilemaps_for_maps;
    pub use gen_tilemaps_for_maps::*;

    mod load_first_map;
    pub use load_first_map::*;

    mod update_tilemaps;
    pub use update_tilemaps::*;
}

mod loaded_tilesets;
mod map;
mod map_plugin;
mod paths;

pub mod prelude {
    mod internal {
        pub use bevy::asset::FileAssetIo;
        pub use bevy::ecs::{bundle, schedule::StateData};
        pub use bevy::prelude::*;
        pub use bevy::sprite::Anchor::*;
        pub use bevy::utils::{HashMap, HashSet};

        pub use banana_bevy_utils::prelude::*;

        pub use iyes_loopless::prelude::*;

        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tileset::prelude::*;

        pub use smart_default::SmartDefault;

        pub use atrl_common::prelude::*;
    }
    pub(crate) use internal::*;

    mod external {
        pub use crate::functions::*;
        pub use crate::procgen::*;
        pub use crate::systems::*;

        pub use crate::loaded_tilesets::*;
        pub use crate::map::*;
        pub use crate::map_plugin::*;
        pub use crate::paths::*;
    }
    pub use external::*;
}
