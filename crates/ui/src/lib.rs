mod main_menu {
    mod systems {
        mod setup_main_menu;
        pub use setup_main_menu::*;
    }
    pub use systems::*;

    mod main_menu_plugin;
    pub use main_menu_plugin::*;
}

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

mod ui_plugin;

pub mod prelude {
    mod import {
        pub use atrl_common::prelude::*;
        pub use atrl_data::prelude::*;
        pub use atrl_raws::prelude::*;
        pub use bevy::{app::AppExit, prelude::*};
        pub use iyes_loopless::prelude::*;
        pub use kayak_ui::prelude::*;
        pub use kayak_ui::widgets::*;
    }
    pub(crate) use import::*;

    pub use crate::main_menu::*;
    pub use crate::ui_plugin::*;
    pub use crate::widgets::*;
}
