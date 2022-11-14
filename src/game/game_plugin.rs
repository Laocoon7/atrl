use crate::game::prelude::internal::*;
use crate::prelude::*;

pub struct GamePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to
    /// [`state_construct`]
    pub state_construct: T,
    pub state_main_menu: T,
    pub state_running: T,
}

impl<T: StateNext> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameContext>()
            .add_plugin(CameraPlugin { state_running: self.state_running.clone() })
            .add_plugin(MapPlugin {
                state_running: self.state_running.clone(),
                state_construct: self.state_construct.clone(),
            })
            .add_plugin(SpawnerPlugin { state_running: self.state_running.clone() })
            .add_plugin(PlayerPlugin { state_running: self.state_running.clone() })
            .add_plugin(UiPlugin { state_main_menu: self.state_main_menu.clone() });
    }
}
