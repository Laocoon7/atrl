#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

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

mod raw_master {
    mod templates {
        mod actor_template;
        pub use actor_template::*;
        mod base_raw_component;
        pub use base_raw_component::*;
        mod stat_templates;
        pub use stat_templates::*;
    }
    pub use templates::*;

    mod utils {
        mod spawn;
        pub use spawn::*;
    }
    pub use utils::*;

    mod raw_master;
    pub use raw_master::*;
    mod raws;
    pub use raws::*;
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
    mod wait_for_tilesets_to_load;
    pub use wait_for_tilesets_to_load::*;
}

mod raw_plugin;

pub mod prelude {
    mod internal {
        pub use crate::systems::*;
    }
    pub(crate) use internal::*;
    mod import {
        pub use atrl_data::prelude::{embedded_resource, *};
        pub use bevy::{
            prelude::*,
            utils::hashbrown::{HashMap, HashSet},
        };
        pub use bevy_tileset::prelude::*;
        pub use big_brain::prelude::*;
        pub use iyes_loopless::prelude::*;
        pub use iyes_progress::prelude::*;
        pub use kayak_ui::{prelude::*, widgets::*};
        pub use leafwing_input_manager::prelude::*;
        pub use serde::Deserialize;
        pub use smart_default::SmartDefault;
    }
    pub(crate) use import::*;
    mod export {
        pub use crate::{raw_master::*, raw_plugin::*, resources::*};
    }
    pub use export::*;
}
