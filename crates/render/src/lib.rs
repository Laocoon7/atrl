#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

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
        }
        pub use systems::*;

        mod menu_button;
        pub use menu_button::*;
        mod menu;
        pub use menu::*;
    }
    pub use widgets::*;

    mod splash {
        mod systems {
            mod setup_splash;
            pub use setup_splash::*;
        }
        pub use systems::*;
        mod splash_plugin;
        pub use splash_plugin::*;
    }
    pub use splash::*;

    mod ui_plugin;
    pub use ui_plugin::*;
}

mod map_renderer_plugin;
mod map_renderer_settings;

#[allow(unused_imports)]
pub mod prelude {
    mod imports {
        pub use atrl_data::prelude::*;
        pub use atrl_raws::prelude::TextureAssets;
        pub use bevy::{
            prelude::*,
            utils::{HashMap, HashSet},
        };
        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tileset::prelude::*;
        pub use iyes_loopless::prelude::*;
        pub use iyes_progress::prelude::*;
        pub use kayak_ui::{prelude::*, widgets::*};
        pub use smart_default::SmartDefault;
    }
    pub(crate) use imports::*;

    pub use crate::{map_renderer_plugin::*, map_renderer_settings::*, ui::*};
}
