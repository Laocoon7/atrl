#![allow(clippy::type_complexity)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

mod functions {
    mod create_tilemap;
    pub use create_tilemap::*;
}

mod systems {
    mod load_tilesets;
    pub use load_tilesets::*;

    mod test_display;
    pub use test_display::*;
}

mod loaded_tilesets;
mod map_plugin;
mod paths;

pub mod prelude {
    mod internal {
        pub use bevy::asset::FileAssetIo;
        pub use bevy::ecs::{bundle, schedule::StateData};
        pub use bevy::prelude::*;
        pub use bevy::sprite::Anchor::*;
        pub use bevy::utils::HashMap;

        pub use banana_bevy_utils::prelude::*;

        pub use iyes_loopless::prelude::*;

        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tile_atlas::*;
        pub use bevy_tileset::prelude::*;

        pub use atrl_common::prelude::*;
    }
    pub(crate) use internal::*;

    mod external {
        pub use super::super::functions::*;
        pub use super::super::loaded_tilesets::*;
        pub use super::super::map_plugin::*;
        pub use super::super::paths::*;
        pub use super::super::systems::*;
    }
    pub use external::*;
}
