mod camera {
    mod main_camera;
    pub use main_camera::*;
}

mod game_plugin;
pub use game_plugin::*;

mod game_state;

pub mod prelude {
    // Bevy
    pub use bevy::prelude::*;
    // Bevy Plugins
    pub use bevy_inspector_egui::prelude::*; // For derive(Inspectable)
    pub use bevy_ecs_tilemap::prelude::*;
    // Serialization
    pub use serde::{Serialize, Deserialize};
    pub use serde_json;

    pub use super::camera::*;
    //pub use super::game_plugin::*; // only needed by main()
    pub use super::game_state::*;
}
