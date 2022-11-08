use crate::prelude::*;

pub struct MapRendererPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapRendererPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(update_map_tiles.run_in_state(self.state_running.clone()));
    }
}
