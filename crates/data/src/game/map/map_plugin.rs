use crate::prelude::*;

#[derive(SystemLabel)]
enum MapLabel {
    SwitchMaps,
}

pub struct MapPlugin<T> {
    pub state_construct: T,
    pub state_running: T,
}

impl<T: StateNext> MapPlugin<T> {
    #[inline(always)]
    pub const fn new(state_construct: T, state_running: T) -> Self {
        Self {
            state_construct,
            state_running,
        }
    }
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_construct, startup_map_manager)
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running)
                    .label(MapLabel::SwitchMaps)
                    .with_system(set_current_map_to_current_player)
                    .into(),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running)
                    .after(MapLabel::SwitchMaps)
                    .with_system(update_tilemaps)
                    .into(),
            );
    }
}
