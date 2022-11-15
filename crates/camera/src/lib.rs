mod systems {
    mod spawn_main_camera;
    pub use spawn_main_camera::*;
}

mod camera_plugin;
mod main_camera;

pub mod prelude {
    mod internal {
        pub use bevy::prelude::*;
        pub use bevy::render::camera::{ScalingMode, WindowOrigin};

        pub use atrl_common::prelude::*;

        pub use crate::main_camera::*;
        pub use crate::systems::*;
    }
    pub(crate) use internal::*;

    mod external {
        pub use crate::camera_plugin::*;
    }
    pub use external::*;
}
