use crate::game::prelude::*;

pub struct MapRendererPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapRendererPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_running.clone(), create_renderer)
            .add_system_to_stage(
                CoreStage::Last,
                update_map_tiles.run_in_state(self.state_running.clone()).label("blah"),
            )
            .add_system_to_stage(
                CoreStage::Last,
                draw_map_tiles.run_in_state(self.state_running.clone()).after("blah"),
            );
    }
}
