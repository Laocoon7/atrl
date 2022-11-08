use crate::game::prelude::*;

pub struct MapPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapLoaderPlugin { state_running: self.state_running.clone() })
            .add_plugin(MapRendererPlugin { state_running: self.state_running.clone() });
    }
}
