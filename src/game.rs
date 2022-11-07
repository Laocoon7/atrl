mod camera {
    mod main_camera;
    pub use main_camera::*;
}

mod game_plugin;
pub use game_plugin::*;

mod game_state;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_inspector_egui::prelude::*;
    pub use iyes_loopless::prelude::*;

    pub use super::camera::*;
    //pub use super::game_plugin::*; // only needed by main()
    pub use super::game_state::*;
}
