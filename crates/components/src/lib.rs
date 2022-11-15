#![allow(clippy::module_inception)]

mod ai_component {
    mod ai_component;
    pub use ai_component::*;
    mod ai_type;
    pub use ai_type::*;
}

mod movement {
    mod movement_type;
    pub use movement_type::*;
    mod movement;
    pub use movement::*;
}

mod position {
    mod local_position;
    pub use local_position::*;
    mod world_position;
    pub use world_position::*;
}

mod vision {
    mod vision_type;
    pub use vision_type::*;
    mod vision;
    pub use vision::*;
}

mod consumable;
mod equipable;
mod equipment_slots;
mod health;
mod tags;

pub mod prelude {
    mod import {
        pub use atrl_common::prelude::*;
        pub use banana_bevy_utils::prelude::*;
        pub use bevy::prelude::*;
        pub use serde::{Deserialize, Serialize};

        #[cfg(feature = "debug")]
        pub use bevy_inspector_egui::prelude::Inspectable;
    }
    pub(crate) use import::*;

    mod export {
        pub use crate::ai_component::*;
        pub use crate::consumable::*;
        pub use crate::equipable::*;
        pub use crate::equipment_slots::*;
        pub use crate::health::*;
        pub use crate::movement::*;
        pub use crate::position::*;
        pub use crate::tags::*;
        pub use crate::vision::*;
    }
    pub use export::*;
}
