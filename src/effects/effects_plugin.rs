use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct ProcessEffectsPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> ProcessEffectsPlugin<T> {
    fn setup_effect_stages(self, app: &mut App) -> Self {
        use CoreStage::*;
        app.add_stage_after(
            AtrlStage::ProcessTurns,
            AtrlStage::ProcessEffects,
            SystemStage::parallel(),
        );
        self
    }
}

impl<T: StateNext> Plugin for ProcessEffectsPlugin<T> {
    fn build(&self, app: &mut App) {
        self.setup_effect_stages(app);
        app.add_system_set_to_stage(
            AtrlStage::ProcessEffects,
            ConditionSet::new().with_system(process_effects).into(),
        );
    }
}
