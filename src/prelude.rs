// Main Prelude for anything in the ATRL crate

// Pull in from extern crate's here
// This includes atrl::crates::*;
mod import {
    pub use bevy::{
        ecs::system::SystemState,
        math::Vec3Swizzles,
        prelude::*,
        utils::{HashMap, HashSet},
        window::{PresentMode, WindowDescriptor, WindowResizeConstraints},
    };

    pub use bevy_asset_loader::prelude::*;

    pub use leafwing_input_manager::{action_state::ActionState, prelude::*};

    pub use serde::{Deserialize, Serialize};

    pub use iyes_loopless::prelude::*;
    pub use iyes_progress::prelude::*;

    pub use banana_bevy_utils::prelude::*;

    pub use rand::prelude::*;

    pub use big_brain::prelude::*;

    pub use num_derive::{FromPrimitive, ToPrimitive};

    pub use num_traits::ToPrimitive;

    pub use once_cell::sync::Lazy;

    pub use parking_lot::{Mutex, MutexGuard};

    pub use smart_default::SmartDefault;

    #[cfg(feature = "debug")]
    pub use bevy_inspector_egui::prelude::*;

    #[cfg(feature = "parallel")]
    pub use rayon::prelude::*;

    pub use atrl_common::prelude::AssetLoadStates::*;
    pub use atrl_common::prelude::ConstructStates::*;
    pub use atrl_common::prelude::UiStates::*;
    pub use atrl_common::prelude::*;
    pub use atrl_components::prelude::*;
    pub use atrl_raws::prelude::*;
}
pub use import::*;

// Push out from here.
mod export {
    pub use crate::app_settings::*;

    pub use crate::game::prelude::external::*;
    pub use crate::procgen::prelude::external::*;
}
pub use export::*;
