use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum AtrlStage {
    ConsumeEvents,
}

pub struct EcsPlugin<T, R> {
    pub state_running: T,
    pub turn_state_ticking: R,
}

impl<T: StateNext, R: StateNext + Resource> EcsPlugin<T, R> {
    pub fn setup_stages(app: &mut App) {
        app.add_stage_after(CoreStage::Update, AtrlStage::ConsumeEvents, SystemStage::parallel());
    }
}

impl<T: StateNext, R: StateNext + Resource> Plugin for EcsPlugin<T, R> {
    fn build(&self, app: &mut App) {
        Self::setup_stages(app);

        app.add_plugin(AIPlugin {
            state_running: self.state_running.clone(),
            turn_state_ticking: self.turn_state_ticking.clone(),
        });

        app.add_system_set_to_stage(
            AtrlStage::ConsumeEvents,
            ConditionSet::new()
                .run_in_state(self.state_running.clone())
                .run_if_resource_equals(self.turn_state_ticking.clone())
                .with_system(fov)
                .into(),
        );
    }
}
