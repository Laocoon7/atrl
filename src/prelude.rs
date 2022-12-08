// Main Prelude for anything in the ATRL crate
// Pull in from extern crate's here
// This includes atrl::crates::*;
mod import {
    pub use atrl_data::{
        impl_as_primative, impl_default, impl_new, insert_resource,
        prelude::{AssetLoadState::*, ConstructState::*, UiState::*, *},
        remove_resource, spawn_component, switch_game_state,
    };
    pub use atrl_raws::prelude::*;
    pub use atrl_renderer::prelude::*;
    pub use atrl_saveload::prelude::*;
    pub use bevy::{
        app::AppExit,
        core_pipeline::clear_color::ClearColorConfig,
        ecs::system::{SystemParam, SystemState},
        math::Vec3Swizzles,
        prelude::*,
        render::{
            camera::{RenderTarget, ScalingMode, Viewport, WindowOrigin},
            once_cell::sync::Lazy,
        },
        utils::{HashMap, HashSet},
        window::{PresentMode, WindowDescriptor, WindowResizeConstraints},
    };
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bevy_tileset::prelude::*;
    pub use big_brain::{actions::ActionState as BigBrainActionState, prelude::*};
    pub use index_list::{Index, IndexList};
    pub use iyes_loopless::prelude::*;
    pub use kayak_ui::{prelude::*, widgets::*};
    pub use leafwing_input_manager::{action_state::ActionState, prelude::*};
    pub use num_traits::*;
    pub use rand::{distributions::Uniform, prelude::*};
    pub use rand_pcg::Pcg64;
    pub use smart_default::SmartDefault;

    #[cfg(feature = "debug")]
    mod egui {
        pub use bevy_inspector_egui::{
            bevy_egui::EguiPlugin, bevy_inspector::hierarchy::SelectedEntities, egui, prelude::*, quick::*,
            DefaultInspectorConfigPlugin,
        };
    }
    pub use atrl_data::prelude::{AssetLoadState::*, ConstructState::*, UiState::*, *};
    pub use atrl_renderer::prelude::*;
    #[cfg(feature = "debug")]
    pub use {crate::debug::*, egui::*};
}
pub use import::*;

pub use crate::{
    ai::*, camera::*, ecs::*, effects::*, events::*, game_plugin::*, log::*, player::*, spawner::*, turn::*,
};
