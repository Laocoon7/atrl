#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(unused_imports)] // TODO: REMOVE ME

mod resources {
    mod loaded_tilesets;
    pub use loaded_tilesets::*;
    mod loaded_fonts;
    pub use loaded_fonts::*;
    mod texture_assets;
    pub use texture_assets::*;
    mod asset_settings;
    pub use asset_settings::*;
}

mod systems {
    mod load_tilesets;
    pub use load_tilesets::*;
    mod load_fonts;
    pub use load_fonts::*;
    mod load_textures;
    pub use load_textures::*;
    mod check_loaded_assets;
    pub use check_loaded_assets::*;
}

mod raw_plugin;

pub mod prelude {
    mod internal {
        pub use crate::systems::*;
    }
    pub(crate) use internal::*;

    mod import {
        pub use atrl_common::prelude::*;
        pub use atrl_data::prelude::*;

        pub use bevy::prelude::*;
        pub use iyes_loopless::prelude::*;
        pub use iyes_progress::prelude::*;

        pub use kayak_ui::prelude::*;
        pub use kayak_ui::widgets::*;

        pub use bevy_tileset::prelude::*;

        pub use smart_default::SmartDefault;
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::raw_plugin::*;
        pub use crate::resources::*;
    }
    pub use export::*;
}
