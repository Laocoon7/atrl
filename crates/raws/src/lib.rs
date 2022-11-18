mod systems {
    mod check_loaded_assets;
    mod splash;
    pub use check_loaded_assets::*;
    pub use splash::*;
}

mod font_assets;
mod raw_plugin;
mod splash_plugin;
mod texture_assets;

pub mod prelude {
    mod internal {
        pub use crate::systems::*;
    }
    pub(crate) use internal::*;

    mod import {
        pub use atrl_common::prelude::*;
        pub use atrl_data::prelude::*;
        pub use bevy::prelude::*;
        pub use bevy_asset_loader::prelude::*;
        pub use iyes_loopless::prelude::*;
        pub use iyes_progress::prelude::*;
        pub use kayak_ui::prelude::*;
        pub use kayak_ui::widgets::*;
        pub use smart_default::SmartDefault;
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::font_assets::*;
        pub use crate::raw_plugin::*;
        pub use crate::splash_plugin::*;
        pub use crate::texture_assets::*;
    }
    pub use export::*;
}
