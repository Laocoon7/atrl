#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(unused_imports)] // TODO: REMOVE ME

mod components {
    mod tileset_identity;
    pub use tileset_identity::*;
}

mod functions {
    mod create_tilemap;
    pub use create_tilemap::*;
}

mod resources {
    //mod loaded_tilesets;
    mod loaded_tilesets;
    pub use loaded_tilesets::*;
}

mod systems {
    mod load_tilesets;
    pub use load_tilesets::*;
}

mod ui {
    mod main_menu {
        mod systems {
            mod setup_main_menu;
            pub use setup_main_menu::*;
        }
        pub use systems::*;

        mod main_menu_plugin;
        pub use main_menu_plugin::*;
    }
    pub use main_menu::*;

    mod widgets {
        mod systems {
            mod menu_button_render;
            pub use menu_button_render::*;
            mod menu_render;
            pub use menu_render::*;
        }
        pub use systems::*;

        mod menu_button;
        pub use menu_button::*;
        mod menu;
        pub use menu::*;
    }
    pub use widgets::*;

    mod ui_plugin;
    pub use ui_plugin::*;
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

        pub use kayak_ui::prelude::*;
        pub use kayak_ui::widgets::*;

        pub use atrl_data::prelude::CurrentGameState;
        pub use atrl_raws::prelude::TextureAssets;
    }
    pub(crate) use imports::*;

    pub use crate::components::*;
    pub use crate::functions::*;
    pub use crate::resources::*;
    pub use crate::systems::*;
    pub use crate::ui::*;

    pub use crate::map_renderer_plugin::*;
    pub use crate::map_renderer_settings::*;
}
