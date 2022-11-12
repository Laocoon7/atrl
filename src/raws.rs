mod systems {
    mod check_loaded_assets;
    pub use check_loaded_assets::*;
    mod splash;
    pub use splash::*;
}
pub use systems::*;

mod font_assets;
pub use font_assets::*;
mod raw_plugin;
pub use raw_plugin::*;
pub mod splash_plugin;
pub use splash_plugin::*;
mod texture_assets;
pub use texture_assets::*;
