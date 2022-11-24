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
    pub use kayak_ui::{prelude::*, widgets::*};
    #[cfg(feature = "debug")]
    mod egui {
        pub use bevy_inspector_egui::{
            bevy_egui::EguiPlugin, bevy_inspector::hierarchy::SelectedEntities, egui, prelude::*, quick::*,
            DefaultInspectorConfigPlugin,
        };
    }
    pub use atrl_camera::prelude::*;
    pub use atrl_common::prelude::*;
    pub use atrl_data::prelude::{AssetLoadState::*, ConstructState::*, TurnState::*, UiState::*, *};
    pub use atrl_game::prelude::*;
    pub use atrl_renderer::prelude::*;
    #[cfg(feature = "debug")]
    pub use {crate::debug::*, egui::*};
}
pub use import::*;
