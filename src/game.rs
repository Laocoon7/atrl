mod game_plugin;
pub use game_plugin::*;

mod game_state;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use iyes_loopless::prelude::*;

    pub use super::game_state::*;
}
