use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum AtrlStage {
    Startup,
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
            state_running: self.state_running,
            turn_state_ticking: self.turn_state_ticking,
        });

        // We need a `Startup` set to run all the initial systems
        app.add_enter_system_set(self.state_running, ConditionSet::new().with_system(fov).into());

        app.add_system_set_to_stage(
            AtrlStage::ConsumeEvents,
            ConditionSet::new()
                .run_in_state(self.state_running)
                .run_if_resource_equals(self.turn_state_ticking)
                .with_system(fov)
                .into(),
        );
    }
}
