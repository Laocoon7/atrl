use super::systems::*;
use crate::game::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum MapSystem {
    Update,
    Draw,
}

pub struct MapPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_running.clone(), create_renderer)
            .add_enter_system(self.state_running.clone(), load_first_map)
            .add_system_to_stage(
                CoreStage::Last,
                update_map_tiles
                    .run_in_state(self.state_running.clone())
                    .label(MapSystem::Update)
                    .before(MapSystem::Draw),
            )
            .add_system_to_stage(
                CoreStage::Last,
                draw_map_tiles.run_in_state(self.state_running.clone()).label(MapSystem::Draw),
            );
    }
}
