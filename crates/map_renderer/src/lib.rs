#![allow(clippy::type_complexity)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

mod components {
    mod tileset_identity;
    pub(crate) use tileset_identity::*;
}

mod functions {
    mod create_tilemap;
    pub use create_tilemap::*;
}

mod resources {
    //mod loaded_tilesets;
    mod loaded_tilesets;
    pub(crate) use loaded_tilesets::*;
}

mod systems {
    mod load_tilesets;
    pub(crate) use load_tilesets::*;
}

mod map_renderer_plugin;
mod map_renderer_settings;

#[allow(unused_imports)]
pub mod prelude {
    mod imports {
        pub use bevy::{
            prelude::*,
            utils::{HashMap, HashSet},
        };

        pub use bevy_ecs_tilemap::prelude::*;

        pub use bevy_tileset::prelude::*;

        pub use iyes_loopless::prelude::*;

        pub use atrl_common::prelude::*;
    }
    pub(crate) use imports::*;

    pub use crate::components::*;
    pub use crate::functions::*;
    pub use crate::resources::*;
    pub use crate::systems::*;

    pub use crate::map_renderer_plugin::*;
    pub use crate::map_renderer_settings::*;
}
