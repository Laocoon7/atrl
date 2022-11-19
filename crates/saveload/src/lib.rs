#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(unused_imports)] // TODO: REMOVE ME

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
