mod systems {}

mod saveload_plugin;

#[allow(unused_imports)] // TODO: Remove this once crate::internal::* has a reference
pub mod prelude {
    mod internal {
        pub use crate::systems::*;
    }
    pub(crate) use internal::*;

    mod import {
        pub use atrl_common::prelude::*;
        pub use atrl_data::prelude::*;
        pub use bevy::prelude::*;
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::saveload_plugin::*;
    }
    pub use export::*;
}
