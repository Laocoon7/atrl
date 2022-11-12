mod systems {
    mod check_loaded_assets;
    pub use check_loaded_assets::*;
    mod splash;
    pub use splash::*;
}

mod font_assets;
mod raw_plugin;
mod splash_plugin;
mod texture_assets;

pub mod prelude {
    // Files inside of atrl::raws *may*
    // use crate::raws::prelude::internal::*;
    // Files outside of atrl::raws should only
    // access raws from crate::prelude::*;
    pub mod internal {
        pub use super::super::font_assets::*;
        pub use super::super::raw_plugin::*;
        pub use super::super::splash_plugin::*;
        pub use super::super::systems::*;
        pub use super::super::texture_assets::*;
    }
    // No pub use here, explicit folder for Raws internal data
    pub mod external {
        pub use super::super::raw_plugin::*;
        pub use super::super::texture_assets::*;
    }
}
