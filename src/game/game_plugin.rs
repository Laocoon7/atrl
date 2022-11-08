use crate::prelude::*;

pub struct GamePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to
    /// [`state_construct`]
    pub state_asset_load: T,
    pub state_construct: T,
    pub state_main_menu: T,
    pub state_running: T,
}

impl<T: StateNext> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Random>()
            .add_plugin(CameraPlugin)
            .add_plugin(MapPlugin { state_running: self.state_running.clone() })
            .add_plugin(ProcgenPlugin {
                state_construct: self.state_construct.clone(),
                state_running: self.state_running.clone(),
            });
    }
}
