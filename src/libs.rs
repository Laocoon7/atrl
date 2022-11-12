// Just used as a list of libraries.
// We shouldn't be pulling from here.

pub use bevy;
// Rename bevy sub modules
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

pub use banana_bevy_utils;
pub use bevy_asset_loader;
pub use bevy_common_assets;
pub use bevy_egui;
pub use bevy_tweening;
pub use big_brain; //big-brain;
pub use iyes_loopless;
pub use iyes_progress;
pub use leafwing_input_manager; //leafwing-input-manager;
pub use noise;
pub use num_derive;
pub use num_traits;
pub use once_cell;
pub use parking_lot;
pub use rand;
pub use rand_pcg;
pub use rand_seeder;
pub use ron;
pub use serde;
pub use smart_default; //smart-default;
pub use strum;
pub use xxhash_rust; //xxhash-rust;

///////////////////
// Specific to ATRL (Debug/Optional/Release)
///////////////////
#[cfg(feature = "release")]
pub use bevy_embedded_assets;
#[cfg(feature = "debug")]
pub use bevy_inspector_egui;
#[cfg(feature = "debug")]
pub use bevy_mod_debugdump;
#[cfg(feature = "debug")]
pub use colored;
#[cfg(feature = "parallel")]
pub use rayon;
