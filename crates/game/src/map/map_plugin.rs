use crate::prelude::*;

pub struct MapPlugin<T> {
    pub state_construct: T,
    pub state_running: T,
}

impl<T: StateNext> MapPlugin<T> {
    #[inline(always)]
    pub const fn new(state_construct: T, state_running: T) -> Self {
        Self { state_construct, state_running }
    }
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapManager::default())
            .add_exit_system_set(
                self.state_construct,
                ConditionSet::new().with_system(load_first_map).into(),
            )
            // TODO: Move to raws???
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(self.state_construct)
                    .with_system(wait_for_tilesets_to_load)
                    .into(),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running)
                    .with_system(update_tilemaps)
                    .into(),
            );
    }
}
