use crate::prelude::*;

pub struct GameMapPlugin<T> {
    pub state_construct: T,
    pub state_running: T,
}

impl<T: StateNext> GameMapPlugin<T> {
    pub fn new(state_construct: T, state_running: T) -> Self {
        Self { state_construct, state_running }
    }
}

impl<T: StateNext> Plugin for GameMapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapManager::new())
            .add_exit_system_set(
                self.state_construct.clone(),
                ConditionSet::new().with_system(load_first_map).into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(self.state_construct.clone())
                    .with_system(wait_for_tilesets_to_load)
                    .into(),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .with_system(update_tilemaps)
                    .into(),
            );
    }
}
