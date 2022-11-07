mod camera {
    mod main_camera;
    pub use main_camera::*;
}

mod game_plugin;
pub use game_plugin::*;

mod game_state;

mod random {
    mod noise;
    pub use self::noise::*;
    mod prht;
    pub use prht::*;
    mod prng;
    pub use prng::*;
    mod random;
    pub use random::*;
}

mod utils {
    mod range;
    pub use range::*;
}

pub mod prelude {
    // Bevy
    pub use bevy::prelude::*;
    // Bevy Plugins
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bevy_inspector_egui::prelude::*; // For derive(Inspectable)
                                             // Serialization
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;

    pub use super::camera::*;
    //pub use super::game_plugin::*; // only needed by main()
    pub use super::game_state::*;
    pub use super::random::*;
    pub use super::utils::*;
}
