pub use bevy::prelude::*;

// BEVY :)
pub use bevy;
pub use bevy::app as bevy_app;
pub use bevy::asset as bevy_asset;
pub use bevy::core as bevy_core;
pub use bevy::diagnostic as bevy_diagnostic;
pub use bevy::ecs as bevy_ecs;
pub use bevy::input as bevy_input;
pub use bevy::log as bevy_log;
pub use bevy::math as bevy_math;
pub use bevy::reflect as bevy_reflect;
pub use bevy::scene as bevy_scene;
pub use bevy::tasks as bevy_tasks;
pub use bevy::transform as bevy_transform;
pub use bevy::utils as bevy_utils;
pub use bevy::window as bevy_window;

// States
pub use iyes_loopless;
pub use iyes_loopless::prelude::*;
pub use iyes_progress;
pub use iyes_progress::prelude::*;

// Assets
pub use bevy_common_assets;
pub use bevy_tweening;

// Tilemaps
pub use bevy_ecs_tilemap;

// Debug + UI
pub use bevy_egui;
pub use bevy_inspector_egui;

// Utility
pub use big_brain;

#[cfg(feature = "parallel")]
pub use rayon;
