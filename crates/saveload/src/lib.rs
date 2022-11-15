mod systems {}

mod saveload_plugin;

pub mod prelude {
    mod internal {
        pub use crate::systems::*;
    }
    pub(crate) use internal::*;

    mod import {
        pub use atrl_common::prelude::*;
        pub use atrl_map::prelude::Map;
        pub use bevy::prelude::*;
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::saveload_plugin::*;
    }
    pub use export::*;
}
