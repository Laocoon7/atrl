use crate::prelude::*;

const UPDATE_TILES_SYSTEM: &str = "update_map_tiles";

pub struct MapRendererPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapRendererPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_running.clone(), create_renderer)
            .add_system_to_stage(
                CoreStage::Last,
                update_map_tiles
                    .run_in_state(self.state_running.clone())
                    .label(UPDATE_TILES_SYSTEM),
            )
            .add_system_to_stage(
                CoreStage::Last,
                draw_map_tiles.run_in_state(self.state_running.clone()).after(UPDATE_TILES_SYSTEM),
            );
    }
}
