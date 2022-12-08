use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct SystemsPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> SystemsPlugin<T> {
    fn setup_startup_systems(self, app: &mut App) -> Self {
        // Startup
        app.add_enter_system_set(
            self.state_running,
            ConditionSet::new().with_system(fov).into(),
        );
        self
    }
}

impl<T: StateNext> Plugin for SystemsPlugin<T> {
    fn build(&self, app: &mut App) {
        self.setup_startup_systems(app);

        app.add_plugin(ProccessTurnsPlugin {
            state_running: self.state_running,
        })
        .add_plugin(ProcessEffectsPlugin {
            state_running: self.state_running,
        })
        .add_plugin(ProccessEventsPlugin {
            state_running: self.state_running,
        });

        app.add_system_set_to_stage(
            CoreStage::First,
            ConditionSet::new().run_in_state(self.state_running).with_system(update_mouse_position).into(),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .label("cull_dead")
                .run_in_state(self.state_running)
                .with_system(cull_dead)
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .after("cull_dead")
                .run_in_state(self.state_running)
                .with_system(fov)
                .with_system(update_targeting)
                .into(),
        );
    }
}
