mod loader {}

mod renderer {
    mod animated_tile;
    pub use animated_tile::*;
    mod animation_tile_bundle;
    pub use animation_tile_bundle::*;
    mod background_entity_holder;
    pub use background_entity_holder::*;
    mod background_tile;
    pub use background_tile::*;
    mod background_tile_bundle;
    pub use background_tile_bundle::*;
    mod foreground_tile;
    pub use foreground_tile::*;
    mod foreground_tile_bundle;
    pub use foreground_tile_bundle::*;
    mod map_context;
    pub use map_context::*;
    mod map_renderer;
    pub use map_renderer::*;
    mod render_actions;
    pub use render_actions::*;
}

mod resources {
    mod theme_server;
    pub use theme_server::*;
}

mod systems {
    mod load_themes_into_theme_server;
    pub use load_themes_into_theme_server::*;
    mod redraw_map_renderers;
    pub use redraw_map_renderers::*;
    mod update_animations;
    pub use update_animations::*;
}

mod themes {
    mod builders {
        mod animation_builder;
        pub use animation_builder::*;
        mod color_definition_builder;
        pub use color_definition_builder::*;
        mod frame_builder;
        pub use frame_builder::*;
        mod theme_builder;
        pub use theme_builder::*;
        mod tile_builder;
        pub use tile_builder::*;
        mod tileset_builder;
        pub use tileset_builder::*;
    }
    pub use builders::*;
    mod animation;
    pub use animation::*;
    mod color_definition;
    pub use color_definition::*;
    mod frame;
    pub use frame::*;
    #[macro_use]
    mod serialized_object;
    pub use serialized_object::*;
    mod theme;
    pub use theme::*;
    mod tile_like;
    pub use tile_like::*;
    mod tile;
    pub use tile::*;
    mod tileset;
    pub use tileset::*;
}

mod error;
mod file_utils;
mod map_plugin;

mod macros {
    pub use super::impl_serialized_object_for;
}

pub mod prelude {
    mod internal {
        pub use std::path::{Path, PathBuf};

        pub use bevy::ecs::{bundle, schedule::StateData};
        pub use bevy::prelude::*;
        pub use bevy::sprite::Anchor::*;
        pub use bevy::utils::HashMap;

        pub use banana_bevy_utils::prelude::*;

        pub use iyes_loopless::prelude::*;

        pub use ron::ser::{to_string_pretty, to_writer_pretty, PrettyConfig};
        pub use serde::{Deserialize, Serialize};

        pub use atrl_common::prelude::*;

        pub use super::super::loader::*;
        pub use super::super::renderer::*;
        pub use super::super::resources::*;
        pub use super::super::systems::*;

        pub use super::super::error::*;
        pub use super::super::file_utils::*;
        pub use super::super::macros::*;
    }
    pub(crate) use internal::*;

    mod external {
        pub type Result<T> = std::result::Result<T, super::super::error::MyError>;
        pub use super::super::error::MyError;

        pub use super::super::renderer::MapContext;
        pub use super::super::renderer::MapRenderer;

        pub use super::super::map_plugin::*;
        pub use super::super::themes::*;
    }
    pub use external::*;
}
