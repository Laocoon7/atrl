#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

mod resources {
    mod camera_settings_resource;
    pub use camera_settings_resource::*;
    mod loaded_camera;
    pub use loaded_camera::*;
}

mod systems {
    mod spawn_cameras;
    pub use spawn_cameras::*;
}

mod camera_plugin;
mod camera_settings;
mod cameras;

pub mod prelude {
    mod imports {
        pub use bevy::{
            core_pipeline::clear_color::ClearColorConfig,
            ecs::system::SystemParam,
            prelude::*,
            render::camera::{RenderTarget, ScalingMode, Viewport, WindowOrigin},
            utils::HashMap,
        };
        pub use iyes_loopless::prelude::*;
    }
    pub(crate) use imports::*;

    // `LoadedCameras` must be public for `Cameras` SystemParam
    // Don't add `LoadedCameras` to external prelude...
    pub(crate) use crate::resources::*;
    // Systems should be loaded by the plugin...
    // Don't add systems to the external prelude...
    pub(crate) use crate::systems::*;

    pub use crate::camera_plugin::*;
    pub use crate::camera_settings::*;
    pub use crate::cameras::*;
}
