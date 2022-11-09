use super::systems::*;
use crate::game::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum MapSystem {
    ChangeTheme,
    Update,
    Draw,
}

pub struct MapPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapLoader>()
            .init_resource::<TileLoader>()
            .add_enter_system(self.state_running.clone(), create_renderer)
            .add_enter_system(self.state_running.clone(), load_first_map)
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .run_if_resource_exists::<ChangeTheme>()
                    .label(MapSystem::ChangeTheme)
                    .before(MapSystem::Update)
                    .with_system(change_map_theme)
                    .into(),
            )
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
