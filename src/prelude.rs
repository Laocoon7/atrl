// Main Prelude for anything in the ATRL crate

// Pull in from extern crate's here
// This includes atrl::crates::*;
mod import {
    pub use bevy::{
        app::AppExit,
        ecs::system::SystemState,
        math::Vec3Swizzles,
        prelude::*,
        utils::{HashMap, HashSet},
        window::{PresentMode, WindowDescriptor, WindowResizeConstraints},
    };

    pub use kayak_ui::prelude::*;
    pub use kayak_ui::widgets::*;

    #[cfg(feature = "debug")]
    mod egui {
        pub use bevy_inspector_egui::bevy_egui::EguiPlugin;
        pub use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
        pub use bevy_inspector_egui::egui;
        pub use bevy_inspector_egui::prelude::*;
        pub use bevy_inspector_egui::{quick::*, DefaultInspectorConfigPlugin};
    }
    #[cfg(feature = "debug")]
    pub use {crate::debug::*, egui::*};

    pub use atrl_camera::prelude::*;
    pub use atrl_common::prelude::*;
    pub use atrl_data::prelude::AssetLoadState::*;
    pub use atrl_data::prelude::ConstructState::*;
    pub use atrl_data::prelude::TurnState::*;
    pub use atrl_data::prelude::UiState::*;
    pub use atrl_data::prelude::*;
    pub use atrl_game::prelude::*;
    pub use atrl_renderer::prelude::*;
}
pub use import::*;
